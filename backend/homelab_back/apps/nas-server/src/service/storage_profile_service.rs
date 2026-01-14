use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::events::UserCreatedEvent;
use homelab_core::storage_profile::StorageProfile;
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait StorageProfileService: Send + Sync {
    async fn save_storage_profile (&self, event: UserCreatedEvent) -> Result<StorageProfile, DataError>;
}

#[derive(new)]
pub struct StorageProfileServiceImpl {
    storage_profile_repo: Arc<dyn StorageProfileRepository>,
}

#[async_trait]
impl StorageProfileService for StorageProfileServiceImpl {
    async fn save_storage_profile(&self, event: UserCreatedEvent) -> Result<StorageProfile, DataError> {
        let profile: StorageProfile = StorageProfile::new(
            event.user_id,
            event.default_storage,
            0i64
        );

        self.storage_profile_repo.create(profile).await
    }
}