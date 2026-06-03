use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::file::File;
use homelab_core::file_label::FileLabel;
use homelab_core::label::Label;
use crate::data::create_file_label_command::CreateFileLabelCommand;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileLabelService: Send + Sync {
    async fn create_file_label(
        &self,
        command: CreateFileLabelCommand,
    ) -> Result<FileLabel, DataError>;
    async fn get_files_by_label(
        &self,
        label_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Vec<File>, DataError>;
    async fn get_labels_by_file(
        &self,
        file_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Vec<Label>, DataError>;
}