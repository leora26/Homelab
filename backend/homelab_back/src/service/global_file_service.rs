use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use crate::db::global_file_repository::GlobalFileRepository;
use crate::domain::global_file::GlobalFile;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait GlobalFileService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<GlobalFile>, DataError>;
}

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