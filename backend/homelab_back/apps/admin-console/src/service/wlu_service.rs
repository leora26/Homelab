use std::sync::Arc;
use actix_web::cookie::time::OffsetDateTime;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use homelab_core::admin_domain::console_wlu::ConsoleWhiteListedUser;
use homelab_core::events::{WhiteListedUserCreatedEvent, WhiteListedUserUpdated};
use crate::db::wlu_repo::WluRepo;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait WluService: Send + Sync {
    async fn log_new_wlu(&self, event: WhiteListedUserCreatedEvent) -> Result<(), DataError>;
    async fn log_updated_wlu(&self, event: WhiteListedUserUpdated) -> Result<(), DataError>;
    async fn get_all(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_all_confirmed(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_all_unconfirmed(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_latest(&self, user_id: Uuid) -> Result<ConsoleWhiteListedUser, DataError>;
    async fn get_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
}

#[derive(new)]
pub struct WluServiceImpl {
    wlu_repo: Arc<dyn WluRepo>
}

#[async_trait]
impl WluService for WluServiceImpl {
    async fn log_new_wlu(&self, event: WhiteListedUserCreatedEvent) -> Result<(), DataError> {
        let logged_wlu = ConsoleWhiteListedUser::new(
            Uuid::new_v4(),
            event.user_id,
            event.email,
            event.full_name,
            false,
            event.created_at,
            OffsetDateTime::now_utc(),
            1
        );

        self.wlu_repo.log_wlu(logged_wlu).await
    }

    async fn log_updated_wlu(&self, event: WhiteListedUserUpdated) -> Result<(), DataError> {
        let logged_wlu = self.wlu_repo.get_latest_wlu(event.user_id)
            .await
            .map_err(|_| DataError::EntityNotFoundException("ConsoleWhitelistedUser".to_string()))?;

        let new_logged_wlu = ConsoleWhiteListedUser::new(
            Uuid::new_v4(),
            event.user_id,
            event.email,
            event.full_name,
            event.is_confirmed,
            logged_wlu.created_at,
            OffsetDateTime::now_utc(),
            logged_wlu.version + 1
        );

        self.wlu_repo.log_wlu(new_logged_wlu).await
    }

    async fn get_all(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        self.wlu_repo.get_all_wlu().await
    }

    async fn get_all_confirmed(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        self.wlu_repo.get_confirmed_wlu().await
    }

    async fn get_all_unconfirmed(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        self.wlu_repo.get_not_confirmed_wlu().await
    }

    async fn get_latest(&self, user_id: Uuid) -> Result<ConsoleWhiteListedUser, DataError> {
        self.wlu_repo.get_latest_wlu(user_id).await
    }

    async fn get_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        self.wlu_repo.get_all_wlu_versions(user_id).await
    }
}