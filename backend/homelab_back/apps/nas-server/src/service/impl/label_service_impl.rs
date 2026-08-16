use crate::data::change_label_command::ChangeLabelCommand;
use crate::data::create_label_command::CreateLabelCommand;
use crate::db::label_repository::{LabelRepository, LabelWithCount};
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::nas_domain::label::Label;
use std::sync::Arc;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use crate::service::contract::label_service::LabelService;

#[derive(new)]
pub struct LabelServiceImpl {
    label_repo: Arc<dyn LabelRepository>,
}

#[async_trait]
impl LabelService for LabelServiceImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Label>, DataError> {
        self.label_repo.get_by_id(id).await
    }

    async fn get_all(&self, owner_id: Uuid) -> Result<Vec<LabelWithCount>, DataError> {
        self.label_repo.get_all(owner_id).await
    }

    async fn create_label(&self, command: CreateLabelCommand) -> Result<Label, DataError> {
        let label = Label::new(
            Uuid::new_v4(),
            command.name,
            command.color,
            command.owner_id,
            OffsetDateTime::now_utc()
        );

        Ok(self.label_repo.create(label).await?)
    }

    async fn delete_label(&self, label_id: Uuid) -> Result<(), DataError> {
        self.label_repo.delete(label_id).await
    }

    async fn change_label(&self, command: ChangeLabelCommand) -> Result<Label, DataError> {
        let mut label = self
            .label_repo
            .get_by_id(command.id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Label".to_string()))?;

        label.update(command.name, command.color);

        Ok(self.label_repo.update(label).await?)
    }
}
