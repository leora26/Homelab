use async_trait::async_trait;
use uuid::Uuid;
use crate::db::global_file_repository::GlobalFileWithMeta;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait GlobalFileService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<GlobalFileWithMeta>, DataError>;
    async fn make_global(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn make_private(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn is_global(&self, file_id: Uuid) -> Result<bool, DataError>;
}