use crate::data::change_label_command::ChangeLabelCommand;
use crate::data::create_label_command::CreateLabelCommand;
use crate::db::label_repository::LabelRepository;
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::label::Label;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait LabelService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Label>, DataError>;
    async fn create_label(&self, command: CreateLabelCommand) -> Result<Label, DataError>;
    async fn delete_label(&self, label_id: Uuid) -> Result<(), DataError>;
    async fn change_label(&self, command: ChangeLabelCommand) -> Result<Label, DataError>;
}

#[derive(new)]
pub struct LabelServiceImpl {
    label_repo: Arc<dyn LabelRepository>,
    storage_profile_repo: Arc<dyn StorageProfileRepository>,
}

#[async_trait]
impl LabelService for LabelServiceImpl {
    async fn get_all(&self) -> Result<Vec<Label>, DataError> {
        self.label_repo.get_all().await
    }

    async fn create_label(&self, command: CreateLabelCommand) -> Result<Label, DataError> {
        let sp = self
            .storage_profile_repo
            .get_by_id(command.owner_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let label = Label::new(Uuid::new_v4(), command.name, command.color, sp.user_id);

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
