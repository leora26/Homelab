use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::label::Label;
use crate::data::change_label_command::ChangeLabelCommand;
use crate::data::create_label_command::CreateLabelCommand;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait LabelService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Label>, DataError>;
    async fn create_label(&self, command: CreateLabelCommand) -> Result<Label, DataError>;
    async fn delete_label(&self, label_id: Uuid) -> Result<(), DataError>;
    async fn change_label(&self, command: ChangeLabelCommand) -> Result<Label, DataError>;
}
