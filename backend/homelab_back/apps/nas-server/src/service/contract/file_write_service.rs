use crate::data::copy_file_command::CopyFileCommand;
use crate::data::init_file_command::InitFileCommand;
use crate::data::move_file_command::MoveFileCommand;
use crate::data::update_file_name_command::UpdateFileNameCommand;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use homelab_core::nas_domain::file::File;

#[async_trait]
pub trait FileWriteService: Send + Sync {
    async fn upload(&self, command: InitFileCommand) -> Result<File, DataError>;
    async fn upload_stream(
        &self,
        file_id: Uuid,
        rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError>;
    async fn update_file_name(
        &self,
        command: UpdateFileNameCommand,
        id: Uuid,
    ) -> Result<File, DataError>;
    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError>;
    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn move_file(&self, command: MoveFileCommand) -> Result<File, DataError>;
    async fn copy_file(&self, command: CopyFileCommand) -> Result<File, DataError>;
    async fn update_stream(
        &self,
        file_id: Uuid,
        rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError>;
    async fn archive_file(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn unarchive_file(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn remove_deleted_file(&self, file_id: Uuid) -> Result<(), DataError>;
}