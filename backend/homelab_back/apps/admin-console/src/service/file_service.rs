use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use homelab_core::admin_domain::console_file::ConsoleFile;
use homelab_core::events::{FileUpdatedEvent, FileUploadedEvent};
use crate::db::file_repo::FileRepo;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileService: Send + Sync {
    async fn log_new_file(&self, event: FileUploadedEvent) -> Result<(), DataError>;
    async fn log_updated_file(&self, event: FileUpdatedEvent) -> Result<(), DataError>;
    async fn get_all_files (&self) -> Result<Vec<ConsoleFile>, DataError>;
    async fn get_latest (&self, file_id: Uuid) -> Result<ConsoleFile, DataError>;
    async fn get_versions (&self, file_id: Uuid) -> Result<Vec<ConsoleFile>, DataError>;
}

#[derive(new)]
pub struct FileServiceImpl {
    file_repo: Arc<dyn FileRepo>
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn log_new_file(&self, event: FileUploadedEvent) -> Result<(), DataError> {
        let logged_file = ConsoleFile::new(
            Uuid::new_v4(),
            event.file_id,
            event.file_type,
            event.is_deleted,
            event.ttl,
            event.size,
            event.upload_status,
            event.created_at,
            OffsetDateTime::now_utc(),
            1
        );

        self.file_repo.log_file(logged_file).await
    }

    async fn log_updated_file(&self, event: FileUpdatedEvent) -> Result<(), DataError> {
        let logged_file = self.file_repo.get_latest_file(event.file_id)
            .await
            .map_err(|_| DataError::EntityNotFoundException("ConsoleFile".to_string()))?;

        let new_logged_file = ConsoleFile::new(
            Uuid::new_v4(),
            logged_file.file_id,
            logged_file.file_type,
            event.is_deleted,
            event.ttl,
            event.size,
            event.upload_status,
            logged_file.created_at,
            OffsetDateTime::now_utc(),
            logged_file.version + 1
        );

        self.file_repo.log_file(new_logged_file).await
    }

    async fn get_all_files(&self) -> Result<Vec<ConsoleFile>, DataError> {
        self.file_repo.get_files().await
    }

    async fn get_latest(&self, file_id: Uuid) -> Result<ConsoleFile, DataError> {
        self.file_repo.get_latest_file(file_id).await
    }

    async fn get_versions(&self, file_id: Uuid) -> Result<Vec<ConsoleFile>, DataError> {
        self.file_repo.get_all_file_versions(file_id).await
    }
}