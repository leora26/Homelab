use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use crate::data::user::create_user_command::CreateUserCommand;
use crate::domain::user::User;
use crate::db::user_repository::UserRepository;
use crate::exception::data_error::DataError;
use crate::types::user_email::UserEmail;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_by_email(&self, email: &String) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, command: CreateUserCommand) -> Result<User, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError>;
    async fn update_password(&self, id: Uuid, pass: &str) -> Result<(), DataError>;
}

#[derive(new)]
pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_by_email(&self, email: &String) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_email(email.to_string()).await
    }

    async fn get_all(&self) -> Result<Vec<User>, DataError> {
        self.user_repo.get_all().await
    }

    async fn create(&self, command: CreateUserCommand) -> Result<User, DataError> {
        let valid_email = UserEmail::parse(command.email)
            .map_err(|e| DataError::ValidationError(e))?;

        let cleaned_name = command.full_name.trim().to_string();

        let u = User::new_complete(Uuid::new_v4(), valid_email.into_inner(), command.password, cleaned_name, command.storage);

        self.user_repo.create(u).await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_id(id).await
    }

    async fn update_password(&self, id: Uuid, pass: &str) -> Result<(), DataError> {
        let mut user = self.user_repo.get_by_id(id).await?
            .ok_or_else(|| DataError::EntityNotFoundException(format!("User not found: {}", id)))?;

        user.set_password(pass);

        self.user_repo.save(user).await?;

        Ok(())
    }
}