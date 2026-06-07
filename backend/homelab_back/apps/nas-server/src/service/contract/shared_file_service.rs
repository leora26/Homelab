use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::nas_domain::shared_file::SharedFile;
use crate::data::create_shared_file_command::CreateSharedFileCommand;
use crate::helpers::data_error::DataError;

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