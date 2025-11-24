use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::white_listed_user::WhiteListedUser;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait WhiteListedUserRepository: Send + Sync {
    async fn create (&self, user: WhiteListedUser) -> Result<WhiteListedUser, DataError>;
    async fn get_all (&self) -> Result<Vec<WhiteListedUser>, DataError>;
    async fn delete_by_id (&self, user_id: Uuid) -> Result<(), DataError>;
    async fn get_by_id (&self, user_id: Uuid) -> Result<Option<WhiteListedUser>, DataError>;
}

pub struct WhiteListedUserRepositoryImpl {
    pool: PgPool,
}

impl WhiteListedUserRepositoryImpl {
    pub fn new (pool: PgPool) -> Self {Self {pool}}
}

#[async_trait]
impl WhiteListedUserRepository for WhiteListedUserRepositoryImpl {
    async fn create(&self, user: WhiteListedUser) -> Result<WhiteListedUser, DataError> {
        let user = sqlx::query_as!(
            WhiteListedUser,
            "INSERT INTO white_listed_users (id, email, full_name, created_at) VALUES ($1, $2, $3, $4) \
            RETURNING id, email, full_name, created_at",
            user.id,
            user.email,
            user.full_name,
            user.created_at
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<WhiteListedUser>, DataError> {
        let users: Vec<WhiteListedUser> = sqlx::query_as!(
            WhiteListedUser,
            "SELECT id, email, full_name, created_at FROM white_listed_users"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn delete_by_id(&self, user_id: Uuid) -> Result<(), DataError> {
        sqlx::query!("DELETE FROM white_listed_users WHERE id = $1", user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<WhiteListedUser>, DataError> {
        let user = sqlx::query_as!(
            WhiteListedUser,
            "SELECT id, email, full_name, created_at FROM white_listed_users \
            WHERE id = $1",
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}