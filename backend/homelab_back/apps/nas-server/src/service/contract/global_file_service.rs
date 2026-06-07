use async_trait::async_trait;
use homelab_core::nas_domain::global_file::GlobalFile;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait GlobalFileService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<GlobalFile>, DataError>;
}