use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use crate::data::create_shared_file_command::CreateSharedFileCommand;
use crate::db::file_repository::FileRepository;
use crate::db::shared_file_repository::{SharedFileRepository};
use crate::db::user_repository::UserRepository;
use homelab_core::file::File;
use homelab_core::shared_file::{SharedFile, SharedFileAccessType};
use homelab_core::user::User;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait SharedFileService: Send + Sync {
    async fn create_shared_file(&self, command: CreateSharedFileCommand) -> Result<SharedFile, DataError>;
    async fn get_all_shared_files_per_user (&self, user_id: Uuid) -> Result<Vec<SharedFile>, DataError>;
}

#[derive(new)]
pub struct SharedFileServiceImpl {
    shared_file_repository: Arc<dyn SharedFileRepository>,
    user_repository: Arc<dyn UserRepository>,
    file_repository: Arc<dyn FileRepository>,
}

#[async_trait]
impl SharedFileService for SharedFileServiceImpl {
    async fn create_shared_file(&self, command: CreateSharedFileCommand) -> Result<SharedFile, DataError> {
        let shared_user: User = self.user_repository.get_by_id(command.user_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let owner_user: User = self.user_repository.get_by_id(command.owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let shared_file: File = self.file_repository.get_by_id(command.file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;


        let shared_file = SharedFile::new(Uuid::new_v4(), shared_file.id, shared_user.id, owner_user.id, SharedFileAccessType::ReadOnly);

        self.shared_file_repository.create_shared_file(&shared_file).await
    }

    async fn get_all_shared_files_per_user(&self, user_id: Uuid) -> Result<Vec<SharedFile>, DataError> {
        self.shared_file_repository.get_all_for_user(user_id).await
    }
}