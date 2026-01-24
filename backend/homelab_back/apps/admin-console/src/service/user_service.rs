use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use homelab_core::admin_domain::console_user::ConsoleUser;
use homelab_core::events::{UserCreatedEvent, UserUpdatedEvent};
use crate::db::user_repo::UserRepo;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn log_new_user(&self, event: UserCreatedEvent) -> Result<(), DataError>;
    async fn log_updated_user(&self, event: UserUpdatedEvent) -> Result<(), DataError>;
    async fn get_all_users(&self) -> Result<Vec<ConsoleUser>, DataError>;
    async fn get_latest(&self, user_id: Uuid) -> Result<ConsoleUser, DataError>;
    async fn get_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleUser>, DataError>;
}

#[derive(new)]
pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepo>
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn log_new_user(&self, event: UserCreatedEvent) -> Result<(), DataError> {
        let logged_user = ConsoleUser::new(
            Uuid::new_v4(),
            event.user_id,
            event.email,
            event.full_name,
            event.default_storage,
            0i64,
            event.created_at,
            OffsetDateTime::now_utc(),
            1
        );
        
        self.user_repo.log_user(logged_user).await
    }

    async fn log_updated_user(&self, event: UserUpdatedEvent) -> Result<(), DataError> {
        let logged_user = self.user_repo.get_latest_user(event.user_id)
            .await
            .map_err(|_| DataError::EntityNotFoundException("ConsoleUser".to_string()))?;
        
        let new_logged_user = ConsoleUser::new(
            Uuid::new_v4(),
            logged_user.user_id,
            logged_user.email,
            logged_user.full_name,
            event.allowed_storage,
            event.taken_storage,
            logged_user.created_at,
            OffsetDateTime::now_utc(),
            logged_user.version + 1
        );
        
        self.user_repo.log_user(new_logged_user).await
    }

    async fn get_all_users(&self) -> Result<Vec<ConsoleUser>, DataError> {
        self.user_repo.get_users().await
    }

    async fn get_latest(&self, user_id: Uuid) -> Result<ConsoleUser, DataError> {
        self.user_repo.get_latest_user(user_id).await
    }

    async fn get_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleUser>, DataError> {
        self.user_repo.get_all_user_versions(user_id).await
    }
}