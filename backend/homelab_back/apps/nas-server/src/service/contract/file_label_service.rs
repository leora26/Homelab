use std::collections::HashMap;
use crate::data::create_file_label_command::CreateFileLabelCommand;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use homelab_core::nas_domain::file::File;
use homelab_core::nas_domain::file_label::FileLabel;
use homelab_core::nas_domain::label::Label;
use uuid::Uuid;

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
    async fn delete_file_label(&self, file_id: Uuid, label_id: Uuid) -> Result<(), DataError>;
    async fn get_labels_for_files(
        &self,
        files: &[Uuid],
        owner_id: Uuid,
    ) -> Result<HashMap<Uuid, Vec<Label>>, DataError>;
}
