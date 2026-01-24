use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::white_listed_user::WhiteListedUser;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait WhiteListedUserRepository: Send + Sync {
    async fn create(&self, user: WhiteListedUser) -> Result<WhiteListedUser, DataError>;
    async fn get_all(&self) -> Result<Vec<WhiteListedUser>, DataError>;
    async fn delete_by_id(&self, user_id: Uuid) -> Result<(), DataError>;
    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<WhiteListedUser>, DataError>;
}

#[derive(new)]
pub struct WhiteListedUserRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl WhiteListedUserRepository for WhiteListedUserRepositoryImpl {
    async fn create(&self, user: WhiteListedUser) -> Result<WhiteListedUser, DataError> {
        let user = sqlx::query_as!(
            WhiteListedUser,
            r#"
            INSERT INTO white_listed_users (id, email, full_name, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, full_name, created_at
            "#,
            user.id,
            user.email,
            user.full_name,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<WhiteListedUser>, DataError> {
        let users: Vec<WhiteListedUser> = sqlx::query_as!(
            WhiteListedUser,
            r#"
            SELECT id, email, full_name, created_at
            FROM white_listed_users
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }

    async fn delete_by_id(&self, user_id: Uuid) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM white_listed_users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<WhiteListedUser>, DataError> {
        let user = sqlx::query_as!(
            WhiteListedUser,
            r#"
            SELECT id, email, full_name, created_at
            FROM white_listed_users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }
}
