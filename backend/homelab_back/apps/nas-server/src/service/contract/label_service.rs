use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::nas_domain::label::Label;
use crate::data::change_label_command::ChangeLabelCommand;
use crate::data::create_label_command::CreateLabelCommand;
use crate::db::label_repository::LabelWithCount;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait LabelService: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Label>, DataError>;
    async fn get_all(&self, owner_id: Uuid) -> Result<Vec<LabelWithCount>, DataError>;
    async fn create_label(&self, command: CreateLabelCommand) -> Result<Label, DataError>;
    async fn delete_label(&self, label_id: Uuid) -> Result<(), DataError>;
    async fn change_label(&self, command: ChangeLabelCommand) -> Result<Label, DataError>;
}
