use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::helpers::data_error::DataError;
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
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::events::rabbitmq::RabbitMqPublisher;
use crate::service::storage_profile_service::StorageProfileService;

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
        let expired_files = self.file_repo.get_expired_files().await?;

        if expired_files.is_empty() {
            return Ok(());
        }

        let mut files_by_owner = HashMap::new();

        for file in expired_files.clone() {
            files_by_owner
                .entry(file.owner_id.clone())
                .or_insert_with(Vec::new)
                .push(file);
        }

        for (owner_id, files) in files_by_owner {
            let total_size_to_reduce: i64 = files.iter().map(|f| f.size).sum();

            self.sp_service.reduce_taken_storage(owner_id, total_size_to_reduce).await?;
        }

        self.remove_deleted_files(expired_files).await
    }
}

impl CleanUpServiceImpl {
    async fn hard_delete_file(&self, file_id: Uuid) -> Result<(), DataError> {
        let file = self
            .file_repo
            .get_deleted_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        self.sp_service.reduce_taken_storage(file.owner_id, file.size).await?;
        self.remove_deleted_files(vec![file]).await
    }

    async fn hard_delete_folder(&self, folder_id: Uuid, user_id: Uuid) -> Result<(), DataError> {
        loop {
            let batch = self.file_repo.get_batch_for_hard_delete_for_folder(folder_id, 10).await?;

            if batch.is_empty() { break;}

            let total_size_to_reduce: i64 = batch.iter().map(|f| f.size).sum();

            self.sp_service.reduce_taken_storage(user_id, total_size_to_reduce).await?;
            self.remove_deleted_files(batch).await?;

            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        Ok(())
    }

    async fn hard_delete_all_users_trash(&self, user_id: Uuid) -> Result<(), DataError> {}

    async fn remove_deleted_files(&self, deleted_files: Vec<File>) -> Result<(), DataError> {
        const CONCURRENCY_LIMIT: usize = 10;

        let results = stream::iter(deleted_files)
            .map(|file| async move {
                let path = file.build_file_path(&self.storage_path);

                let remove_result = match fs::remove_file(&path).await {
                    Ok(_) => Ok(file.id),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        let mut gz_path = path.clone();
                        gz_path.set_extension("gz");

                        match fs::remove_file(gz_path).await {
                            Ok(_) => Ok(file.id),
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

        for res in results {
            match res {
                Ok(id) => success_results.push(id),
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

        Ok(())
    }
}
