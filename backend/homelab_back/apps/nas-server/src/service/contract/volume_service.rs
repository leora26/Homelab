use async_trait::async_trait;
use homelab_core::nas_domain::volume::{ResizeOutcome, VolumeStatus};
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait VolumeService: Send + Sync {
    async fn status(&self) -> Result<VolumeStatus, DataError>;
    async fn resize(&self, requested_bytes: i64, force_shrink: bool) -> Result<ResizeOutcome, DataError>;
}