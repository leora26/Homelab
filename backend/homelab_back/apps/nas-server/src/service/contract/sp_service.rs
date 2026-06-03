use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::events::{UserBlockedEvent, UserCreatedEvent};
use homelab_core::storage_profile::StorageProfile;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait StorageProfileService: Send + Sync {
    async fn save_storage_profile (&self, event: UserCreatedEvent) -> Result<StorageProfile, DataError>;
    async fn toggle_storage_profile(&self, event: UserBlockedEvent) -> Result<(), DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError>;
    async fn reduce_taken_storage(&self, id: Uuid, size: i64) -> Result<(), DataError>;
}
