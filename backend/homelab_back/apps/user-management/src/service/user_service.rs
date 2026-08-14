use crate::data::create_user_command::CreateUserCommand;
use crate::db::user_repository::UserRepository;
use crate::events::rabbitmq::RabbitMqPublisher;
use crate::helpers::data_error::DataError;
use crate::helpers::user_email::UserEmail;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::events::{UserCreatedEvent, UserUpdatedEvent};
use homelab_core::user_domain::user::User;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_by_email(&self, email: &String) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn finalize(&self, command: CreateUserCommand) -> Result<(), DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError>;
    async fn toggle_blocked(&self, id: Uuid, blocked: bool) -> Result<(), DataError>;
    async fn set_storage_quota(&self, id: Uuid, allowed_storage: i64) -> Result<(), DataError>;
}

#[derive(new)]
pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
    publisher: Arc<RabbitMqPublisher>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_by_email(&self, email: &String) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_email(email.to_string()).await
    }

    async fn get_all(&self) -> Result<Vec<User>, DataError> {
        self.user_repo.get_all().await
    }

    async fn finalize(&self, command: CreateUserCommand) -> Result<(), DataError> {

        let user = self.user_repo.get_by_id(command.user_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let valid_email =
            UserEmail::parse(command.email).map_err(|e| DataError::ValidationError(e))?;

        let cleaned_name = command.full_name.trim().to_string();

        let u = User::new_complete(
            user.id,
            valid_email.into_inner(),
            cleaned_name,
            user.external_id
        );

        let event: UserCreatedEvent = UserCreatedEvent::new(
            u.id.clone(),
            u.email.clone(),
            u.full_name.clone(),
            u.created_at.clone(),
            10 * 1024 * 1024 * 1024, // 10GB
        );

        if let Err(e) = self.publisher.publish(&event).await {
            eprintln!("Failed to publish event: {:?}", e);
        }

        self.user_repo.save(u).await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_id(id).await
    }

    async fn toggle_blocked(&self, id: Uuid, blocked: bool) -> Result<(), DataError> {
        let mut user =
            self.user_repo.get_by_id(id).await?.ok_or_else(|| {
                DataError::EntityNotFoundException(format!("User not found: {}", id))
            })?;

        user.toggle_blocked(blocked);

        let updated_event: UserUpdatedEvent = UserUpdatedEvent::new(
            user.id.clone(),
            Some(user.email.clone()),
            Some(user.full_name.clone()),
            None,
            None,
            user.is_blocked.clone(),
        );

        if let Err(e) = self.publisher.publish(&updated_event).await {
            eprintln!("Failed to publish event: {:?}", e);
        }


        self.user_repo.toggle_blocked(user).await?;

        Ok(())
    }

    async fn set_storage_quota(&self, id: Uuid, allowed_storage: i64) -> Result<(), DataError> {
        let user = self.user_repo.get_by_id(id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let event = UserUpdatedEvent::new(
          id,
          None,
          None,
          Some(allowed_storage),
          None,
          user.is_blocked.clone(),
        );

        if let Err(e) = self.publisher.publish(&event).await {
            eprintln!("Failed to publish event: {:?}", e);
        };

        Ok(())
    }
}
