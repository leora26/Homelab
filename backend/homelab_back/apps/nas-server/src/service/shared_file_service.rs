use crate::data::create_shared_file_command::CreateSharedFileCommand;
use crate::db::file_repository::FileRepository;
use crate::db::shared_file_repository::SharedFileRepository;
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::file::File;
use homelab_core::shared_file::{SharedFile, SharedFileAccessType};
use homelab_core::storage_profile::StorageProfile;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait SharedFileService: Send + Sync {
    async fn create_shared_file(
        &self,
        command: CreateSharedFileCommand,
    ) -> Result<SharedFile, DataError>;
    async fn get_all_shared_files_per_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SharedFile>, DataError>;
}

#[derive(new)]
pub struct SharedFileServiceImpl {
    shared_file_repository: Arc<dyn SharedFileRepository>,
    storage_profile_repository: Arc<dyn StorageProfileRepository>,
    file_repository: Arc<dyn FileRepository>,
}

#[async_trait]
impl SharedFileService for SharedFileServiceImpl {
    async fn create_shared_file(
        &self,
        command: CreateSharedFileCommand,
    ) -> Result<SharedFile, DataError> {
        let sp: StorageProfile = self
            .storage_profile_repository
            .get_by_id(command.user_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let owner_sp: StorageProfile = self
            .storage_profile_repository
            .get_by_id(command.owner_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let shared_file: File = self
            .file_repository
            .get_by_id(command.file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let shared_file = SharedFile::new(
            Uuid::new_v4(),
            shared_file.id,
            sp.user_id,
            owner_sp.user_id,
            SharedFileAccessType::ReadOnly,
        );

        self.shared_file_repository
            .create_shared_file(&shared_file)
            .await
    }

    async fn get_all_shared_files_per_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SharedFile>, DataError> {
        self.shared_file_repository.get_all_for_user(user_id).await
    }
}
