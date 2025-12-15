use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use async_trait::async_trait;
use tokio::io::{AsyncWriteExt, BufWriter};
use uuid::Uuid;
use crate::constants::MB;
use crate::data::file_folder::update_file_name_command::UpdateFileNameCommand;
use crate::data::file_folder::init_file_command::InitFileCommand;
use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::db::user_repository::UserRepository;
use crate::domain::file::{File, UploadStatus};
use crate::domain::folder::Folder;
use crate::domain::user::User;
use crate::exception::data_error::{DataError};

#[async_trait]
pub trait FileService: Send + Sync {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted_files(&self) -> Result<Vec<File>, DataError>;
    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError>;
    async fn upload(&self, command: InitFileCommand) -> Result<File, DataError>;
    async fn upload_stream(&self, file_id: Uuid, rx: Receiver<Result<Vec<u8>, DataError>>) -> Result<(), DataError>;
    async fn update_file_name(&self, command: UpdateFileNameCommand, id: Uuid) -> Result<File, DataError>;
    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError>;
    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete(&self, file_id: Uuid) -> Result<(), DataError>;
}

pub struct FileServiceImpl {
    file_repo: Arc<dyn FileRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    user_repo: Arc<dyn UserRepository>,
    storage_path: PathBuf,
}

impl FileServiceImpl {
    pub fn new(
        file_repo: Arc<dyn FileRepository>,
        folder_repo: Arc<dyn FolderRepository>,
        user_repo: Arc<dyn UserRepository>,
        storage_path: PathBuf,
    ) -> Self {
        Self {
            file_repo,
            folder_repo,
            user_repo,
            storage_path
        }
    }
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError> {
        self.file_repo.get_by_id(file_id).await
    }

    async fn get_all_deleted_files(&self) -> Result<Vec<File>, DataError> {
        self.file_repo.get_all_deleted().await
    }

    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError> {
        self.file_repo.search_by_name(format!("%{}%", search_query)).await
    }

    async fn upload(&self, command: InitFileCommand) -> Result<File, DataError> {
        let folder: Folder = self.folder_repo.get_by_id(command.destination).await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        let user: User = self.user_repo.get_by_id(command.owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        if user.validate_storage_size(command.expected_size) {
            let f = File::new(Uuid::new_v4(), command.name, user.id, folder.id, false, command.expected_size);
            self.file_repo.upload(f).await
        } else {
            Err(DataError::NoFreeStorageError)
        }
    }

    async fn upload_stream(&self, file_id: Uuid, mut rx: Receiver<Result<Vec<u8>, DataError>>) -> Result<(), DataError> {
        let mut f = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        if f.upload_status != UploadStatus::Pending {
            return Err(DataError::ValidationError("File is not pending".to_string()));
        }

        let file_path = f.build_file_path(&self.storage_path);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| DataError::IOError(format!("Failed to create buckets: {}", e)))?;
        }

        let file_handle = tokio::fs::File::create(&file_path).await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let mut writer = BufWriter::with_capacity(MB as usize, file_handle);
        let mut total_bytes = 0i64;

        while let Some(chunk_result) = rx.recv().await {
            let data = chunk_result?;

            total_bytes += data.len() as i64;

            if let Err(e) = writer.write_all(&data).await {
                let _ = tokio::fs::remove_file(&file_path).await;
                return Err(DataError::IOError(e.to_string()));
            }
        }

        if let Err(e) = writer.flush().await {
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        if !f.validate_size(total_bytes) {
            f.update_status(UploadStatus::Failed);
            self.file_repo.update(f).await?;
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(DataError::NotMatchingByteSizeError);
        }

        f.update_status(UploadStatus::Completed);
        self.file_repo.update(f).await?;

        Ok(())
    }


    async fn update_file_name(&self, command: UpdateFileNameCommand, file_id: Uuid) -> Result<File, DataError> {
        let mut file: File = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.rename(command.new_name);

        self.file_repo.update(file).await
    }

    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError> {
        let mut file: File = self.file_repo.get_by_id(id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_undeleted();

        self.file_repo.update(file).await
    }

    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError> {
        self.file_repo.delete_all(file_ids).await
    }

    async fn delete(&self, file_id: Uuid) -> Result<(), DataError> {
        let mut file = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_deleted();

        self.file_repo.delete_by_id(file_id).await
    }
}
