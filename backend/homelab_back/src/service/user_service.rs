use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::data::create_user_command::CreateUserCommand;
use crate::domain::user::User;
use crate::db::user_repository::UserRepository;
use crate::exception::data_error::DataError;
use crate::types::user_email::UserEmail;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, command: CreateUserCommand) -> Result<User, DataError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, DataError>;
}

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_email(email).await
    }

    async fn get_all(&self) -> Result<Vec<User>, DataError> {
        self.user_repo.get_all().await
    }

    async fn create(&self, command: CreateUserCommand) -> Result<User, DataError> {

        let valid_email = UserEmail::parse(command.email)
            .map_err(|e| DataError::ValidationError(e))?;

        let cleaned_name = command.full_name.trim().to_string();

        let u = User::new_complete(Uuid::new_v4(), valid_email.into_inner(), command.password, cleaned_name);

        self.user_repo.create(u).await
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, DataError> {
        self.user_repo.get_by_id(id).await
    }
}