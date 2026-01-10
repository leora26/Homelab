use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use crate::data::create_file_label_command::CreateFileLabelCommand;
use crate::db::file_label_repository::FileLabelRepository;
use crate::db::file_repository::FileRepository;
use crate::db::label_repository::LabelRepository;
use crate::db::user_repository::UserRepository;
use homelab_core::file::File;
use homelab_core::file_label::FileLabel;
use homelab_core::label::Label;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileLabelService: Send + Sync {
    async fn create_file_label(&self, command: CreateFileLabelCommand) -> Result<FileLabel, DataError>;
    async fn get_files_by_label(&self, label_id: Uuid, owner_id: Uuid) -> Result<Vec<File>, DataError>;
    async fn get_labels_by_file(&self, file_id: Uuid, owner_id: Uuid) -> Result<Vec<Label>, DataError>;
}

#[derive(new)]
pub struct FileLabelServiceImpl {
    label_repo: Arc<dyn LabelRepository>,
    file_repo: Arc<dyn FileRepository>,
    file_label_repo: Arc<dyn FileLabelRepository>,
    user_repo: Arc<dyn UserRepository>,
}

#[async_trait]
impl FileLabelService for FileLabelServiceImpl {
    async fn create_file_label(&self, command: CreateFileLabelCommand) -> Result<FileLabel, DataError> {
        let fl = FileLabel::new(command.file_id, command.label_id);

        Ok(self.file_label_repo.create(fl).await?)
    }

    async fn get_files_by_label(&self, label_id: Uuid, owner_id: Uuid) -> Result<Vec<File>, DataError> {
        let label = self.label_repo.get_by_id(label_id).await?.
            ok_or_else(|| DataError::EntityNotFoundException("Label".to_string()))?;

        let user = self.user_repo.get_by_id(owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        Ok(self.file_repo.get_all_files_by_label(label.id, user.id).await?)
    }

    async fn get_labels_by_file(&self, file_id: Uuid, owner_id: Uuid) -> Result<Vec<Label>, DataError> {
        let file = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let user = self.user_repo.get_by_id(owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        Ok(self.label_repo.get_labels_by_file(file.id, user.id).await?)
    }
}