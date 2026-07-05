use std::path::PathBuf;
use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::nas_domain::file::File;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileReadService: Send + Sync {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted_files(&self, user_id: Uuid) -> Result<Vec<File>, DataError>;
    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError>;
    async fn get_file_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError>;
    async fn get_file_preview_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError>;
}