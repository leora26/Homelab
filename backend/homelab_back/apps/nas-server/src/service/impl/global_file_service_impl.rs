use crate::db::global_file_repository::{GlobalFileRepository, GlobalFileWithMeta};
use crate::helpers::data_error::DataError;
use crate::service::contract::global_file_service::GlobalFileService;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::nas_domain::global_file::GlobalFile;
use sqlx::types::time::OffsetDateTime;
use std::sync::Arc;
use uuid::Uuid;

#[derive(new)]
pub struct GlobalFileServiceImpl {
    global_file_repository: Arc<dyn GlobalFileRepository>,
}

#[async_trait]
impl GlobalFileService for GlobalFileServiceImpl {
    async fn get_all(&self) -> Result<Vec<GlobalFileWithMeta>, DataError> {
        self.global_file_repository.get_all().await
    }

    async fn make_global(&self, file_id: Uuid) -> Result<(), DataError> {
        if self.global_file_repository.is_global(file_id).await? {
            return Ok(());
        }

        let global_file = GlobalFile::new(Uuid::new_v4(), file_id, OffsetDateTime::now_utc());
        self.global_file_repository.save(global_file).await?;

        Ok(())
    }

    async fn make_private(&self, file_id: Uuid) -> Result<(), DataError> {
        self.global_file_repository.remove_by_original_id(file_id).await
    }

    async fn is_global(&self, file_id: Uuid) -> Result<bool, DataError> {
        self.global_file_repository.is_global(file_id).await
    }
}
