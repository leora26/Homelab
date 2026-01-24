use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::admin_domain::console_user::ConsoleUser;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn log_user(&self, user: ConsoleUser) -> Result<(), DataError>;
    async fn get_users(&self) -> Result<Vec<ConsoleUser>, DataError>;
    async fn get_latest_user(&self, user_id: Uuid) -> Result<ConsoleUser, DataError>;
    async fn get_all_user_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleUser>, DataError>;
}

#[derive(new)]
pub struct UserRepoImpl {
    pool: PgPool,
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn log_user(&self, user: ConsoleUser) -> Result<(), DataError> {
        sqlx::query_as!(
            ConsoleUser,
            r#"
            INSERT INTO console_users (
                                       id,
                                       user_id,
                                       email,
                                       full_name,
                                       allowed_storage,
                                       taken_storage,
                                       created_at,
                                       version
                                       )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.user_id,
            user.email,
            user.full_name,
            user.allowed_storage,
            user.taken_storage,
            user.created_at,
            user.version
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_users(&self) -> Result<Vec<ConsoleUser>, DataError> {
        let users = sqlx::query_as!(
            ConsoleUser,
            r#"
            SELECT
                id,
                user_id,
                email, 
                full_name, 
                allowed_storage, 
                taken_storage, 
                created_at, 
                updated_at, 
                version
            FROM console_users
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }

    async fn get_latest_user(&self, user_id: Uuid) -> Result<ConsoleUser, DataError> {
        let user = sqlx::query_as!(
            ConsoleUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                allowed_storage,
                taken_storage,
                created_at,
                updated_at,
                version
            FROM console_users
            WHERE user_id = $1
            ORDER BY version DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }

    async fn get_all_user_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleUser>, DataError> {
        let users = sqlx::query_as!(
            ConsoleUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                allowed_storage,
                taken_storage,
                created_at,
                updated_at,
                version
            FROM console_users
            WHERE user_id = $1
            ORDER BY version DESC
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }
}
