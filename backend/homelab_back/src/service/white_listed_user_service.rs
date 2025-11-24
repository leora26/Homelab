use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::data::user::create_white_listed_user_command::CreateWhiteListedUserCommand;
use crate::db::user_repository::UserRepository;
use crate::db::white_listed_user_repository::WhiteListedUserRepository;
use crate::domain::user::User;
use crate::domain::white_listed_user::WhiteListedUser;
use crate::exception::data_error::DataError;
use crate::types::user_email::UserEmail;

#[async_trait]
pub trait WhiteListedUserService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<WhiteListedUser>, DataError>;
    async fn confirm(&self, user_id: Uuid) -> Result<User, DataError>;
    async fn create(&self, command: CreateWhiteListedUserCommand) -> Result<WhiteListedUser, DataError>;
}

pub struct WhiteListedServiceImpl {
    white_listed_repo: Arc<dyn WhiteListedUserRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl WhiteListedServiceImpl {
    pub fn new(white_listed_repo: Arc<dyn WhiteListedUserRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self {
            white_listed_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl WhiteListedUserService for WhiteListedServiceImpl {
    async fn get_all(&self) -> Result<Vec<WhiteListedUser>, DataError> {
        self.white_listed_repo.get_all().await
    }

    async fn confirm(&self, user_id: Uuid) -> Result<User, DataError> {
        let wlu_option = self.white_listed_repo.get_by_id(user_id).await?;

        let wlu = wlu_option.ok_or_else(||
            DataError::EntityNotFoundException(format!("WhiteListedUser with id {}", user_id))
        )?;

        let new_user_entity = User::new_pending(
            Uuid::new_v4(),
            wlu.email,
            wlu.full_name,
        );

        let saved_user = self.user_repo.create(new_user_entity)
            .await
            .map_err(|e| DataError::EntityCreationError(format!("White listed user failed creation: {}", e)))?;

        self.white_listed_repo.delete_by_id(user_id).await?;

        Ok(saved_user)
    }

    async fn create(&self, command: CreateWhiteListedUserCommand) -> Result<WhiteListedUser, DataError> {
        let valid_email = UserEmail::parse(command.email)
            .map_err(|e| DataError::ValidationError(e))?;

        let clean_name = command.full_name.trim().to_string();

        let u = WhiteListedUser::new(Uuid::new_v4(), valid_email.into_inner(), clean_name);

        self.white_listed_repo.create(u).await
    }
}