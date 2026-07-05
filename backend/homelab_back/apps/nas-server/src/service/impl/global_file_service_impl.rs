use crate::db::global_file_repository::GlobalFileRepository;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::nas_domain::global_file::GlobalFile;
use std::sync::Arc;
use crate::service::contract::global_file_service::GlobalFileService;

#[derive(new)]
pub struct GlobalFileServiceImpl {
    global_file_repository: Arc<dyn GlobalFileRepository>,
}

#[async_trait]
impl GlobalFileService for GlobalFileServiceImpl {
    async fn get_all(&self) -> Result<Vec<GlobalFile>, DataError> {
        self.global_file_repository.get_all().await
    }
}
