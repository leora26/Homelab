use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use homelab_core::events::{UserCreatedEvent, UserUpdatedEvent};
use homelab_core::nas_domain::storage_profile::StorageProfile;
use homelab_core::nas_domain::storage_stats::StorageStats;
use crate::db::storage_profile_repository::StorageProfileRepository;
use crate::events::rabbitmq::RabbitMqPublisher;
use crate::helpers::data_error::DataError;
use crate::service::contract::sp_service::StorageProfileService;

#[derive(new)]
pub struct StorageProfileServiceImpl {
    storage_profile_repo: Arc<dyn StorageProfileRepository>,
    publisher: Arc<RabbitMqPublisher>,
}

#[async_trait]
impl StorageProfileService for StorageProfileServiceImpl {
    async fn save_storage_profile(&self, event: UserCreatedEvent) -> Result<StorageProfile, DataError> {
        let profile: StorageProfile = StorageProfile::new(
            event.user_id,
            event.default_storage,
            0i64,
            false
        );

        self.storage_profile_repo.create(profile).await
    }

    async fn apply_user_update(&self, event: UserUpdatedEvent) -> Result<(), DataError> {
        let mut sp = self.storage_profile_repo.get_by_id(event.user_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Storage profile".to_string()))?;

        sp.update_sp(
            event.allowed_storage.unwrap_or(sp.allowed_storage),
            event.is_blocked
        );

        self.storage_profile_repo.save(sp).await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError> {
        self.storage_profile_repo.get_by_id(id).await
    }

    async fn reduce_taken_storage(&self, id: Uuid, size: i64) -> Result<(), DataError> {
        let mut sp = self
            .storage_profile_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Storage Profile".to_string()))?;

        sp.reduce_taken_storage_size(size);

        let sp_event: UserUpdatedEvent = UserUpdatedEvent::new(
            sp.user_id.clone(),
            None,
            None,
            Some(sp.allowed_storage.clone()),
            Some(sp.taken_storage.clone()),
            sp.is_blocked.clone(),
        );

        if let Err(e) = self.publisher.publish(&sp_event).await {
            eprintln!("Failed to publish event: {:?}", e);
        }

        self.storage_profile_repo.save(sp).await
    }

    async fn get_storage_stats(&self, id: Uuid) -> Result<StorageStats, DataError> {
        self.storage_profile_repo.get_stats(id).await
    }
}