use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::events::rabbitmq::RabbitMqPublisher;
use crate::helpers::data_error::DataError;
use crate::service::storage_profile_service::StorageProfileService;
use async_trait::async_trait;
use derive_new::new;
use futures::stream::{self, StreamExt};
use homelab_core::events::{DeletionType, TrashCleanUpTriggeredEvent, UserUpdatedEvent};
use homelab_core::file::File;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

#[async_trait]
pub trait CleanUpService: Send + Sync {
    async fn handle_trash_delete(&self, event: TrashCleanUpTriggeredEvent)
        -> Result<(), DataError>;
    async fn hard_delete_all_trash(&self) -> Result<(), DataError>;
}

#[derive(new)]
pub struct CleanUpServiceImpl {
    folder_repo: Arc<dyn FolderRepository>,
    file_repo: Arc<dyn FileRepository>,
    sp_repo: Arc<dyn StorageProfileRepository>,
    sp_service: Arc<dyn StorageProfileService>,
    storage_path: PathBuf,
    publisher: Arc<RabbitMqPublisher>,
}

#[async_trait]
impl CleanUpService for CleanUpServiceImpl {
    async fn handle_trash_delete(
        &self,
        event: TrashCleanUpTriggeredEvent,
    ) -> Result<(), DataError> {
        match event.deletion_type {
            DeletionType::File => {
                let file_id = event
                    .id
                    .ok_or(DataError::InvalidDataError("Missing File ID".to_string()))?;
                self.hard_delete_file(file_id).await
            }
            DeletionType::Folder => {
                let folder_id = event
                    .id
                    .ok_or(DataError::InvalidDataError("Missing Folder ID".to_string()))?;
                self.hard_delete_folder(folder_id, event.user_id).await
            }
            DeletionType::All => self.hard_delete_all_users_trash(event.user_id).await,
        }
    }

    async fn hard_delete_all_trash(&self) -> Result<(), DataError> {
        loop {
            let batch = self.file_repo.get_batch_for_hard_delete(50).await?;

            if batch.is_empty() {
                break;
            }

            let freed_by_user = self.remove_deleted_files(batch).await?;

            if freed_by_user.is_empty() {
                eprintln!("Warning: Stuck on a batch of undeletable files. Aborting cron job");
                break;
            }

            for (owner, freed_size) in freed_by_user {
                if freed_size > 0 {
                    self.sp_service.reduce_taken_storage(owner, freed_size).await?;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }

        self.folder_repo.hard_delete_global_trashed_folders().await?;

        Ok(())
    }
}

impl CleanUpServiceImpl {
    async fn hard_delete_file(&self, file_id: Uuid) -> Result<(), DataError> {
        let file = self
            .file_repo
            .get_deleted_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let freed_size_for_user = self.remove_deleted_files(vec![file.clone()]).await?;
        let freed_size = freed_size_for_user
            .get(&file.owner_id)
            .copied()
            .unwrap_or(0);

        if freed_size > 0 {
            self.sp_service
                .reduce_taken_storage(file.owner_id, freed_size)
                .await?;
        }

        Ok(())
    }

    async fn hard_delete_folder(&self, folder_id: Uuid, user_id: Uuid) -> Result<(), DataError> {
        loop {
            let batch = self
                .file_repo
                .get_batch_for_hard_delete_for_folder(folder_id, 10)
                .await?;

            if batch.is_empty() {
                break;
            }

            let freed_size_for_user = self.remove_deleted_files(batch).await?;
            let freed_size = freed_size_for_user.get(&user_id).copied().unwrap_or(0);

            if freed_size > 0 {
                self.sp_service
                    .reduce_taken_storage(user_id, freed_size)
                    .await?;
            }

            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        self.folder_repo.hard_delete_folder_tree(folder_id).await?;

        Ok(())
    }

    async fn hard_delete_all_users_trash(&self, user_id: Uuid) -> Result<(), DataError> {
        loop {
            let batch = self.file_repo.get_batch_for_user_trash(user_id, 10).await?;

            if batch.is_empty() {
                break;
            }

            let freed_by_user = self.remove_deleted_files(batch).await?;
            let freed_size = freed_by_user.get(&user_id).copied().unwrap_or(0);

            if freed_size > 0 {
                self.sp_service.reduce_taken_storage(user_id, freed_size).await?;
            }

            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        self.folder_repo.hard_delete_all_trashed_folders(user_id).await?;

        Ok(())
    }

    async fn remove_deleted_files(
        &self,
        deleted_files: Vec<File>,
    ) -> Result<HashMap<Uuid, i64>, DataError> {
        const CONCURRENCY_LIMIT: usize = 10;

        let results = stream::iter(deleted_files)
            .map(|file| async move {
                let path = file.build_file_path(&self.storage_path);

                let remove_result = match fs::remove_file(&path).await {
                    Ok(_) => Ok((file.id, file.owner_id, file.size)),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        let mut gz_path = path.clone().into_os_string();
                        gz_path.push(".gz");
                        let gz_path = PathBuf::from(gz_path);

                        match fs::remove_file(gz_path).await {
                            Ok(_) => Ok((file.id, file.owner_id, file.size)),
                            Err(e2) => Err((file.id, e2)),
                        }
                    }
                    Err(e) => Err((file.id, e)),
                };

                if remove_result.is_ok() {
                    if let Some(bucket2) = path.parent() {
                        if fs::remove_dir(bucket2).await.is_ok() {
                            if let Some(bucket1) = bucket2.parent() {
                                let _ = fs::remove_dir(bucket1).await;
                            }
                        }
                    }
                }

                remove_result
            })
            .buffer_unordered(CONCURRENCY_LIMIT)
            .collect::<Vec<_>>()
            .await;

        let mut success_results = Vec::new();
        let mut error_results = Vec::new();
        let mut freed_by_user: HashMap<Uuid, i64> = HashMap::new();

        for res in results {
            match res {
                Ok((id, owner_id, size)) => {
                    success_results.push(id);
                    *freed_by_user.entry(owner_id).or_insert(0) += size;
                }
                Err(e) => error_results.push(e),
            }
        }

        if !success_results.is_empty() {
            self.file_repo.delete_by_ids(&success_results).await?;
        }

        if !error_results.is_empty() {
            return Err(DataError::IOError(format!(
                "Failed to delete {} files from disk. Check logs.",
                error_results.len()
            )));
        }

        Ok(freed_by_user)
    }
}
