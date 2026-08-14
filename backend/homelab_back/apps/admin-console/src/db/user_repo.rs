use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::admin_domain::console_user::ConsoleUser;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn log_user(&self, user: ConsoleUser) -> Result<(), DataError>;
    async fn get_latest(&self, limit: i64, blocked: Option<bool>) -> Result<Vec<ConsoleUser>, DataError>;
    async fn get_log(&self, limit: i64, blocked: Option<bool>) -> Result<Vec<ConsoleUser>, DataError>;
    async fn find_by_query(&self, query: &str) -> Result<Vec<ConsoleUser>, DataError>;
    async fn get_version(&self, query: &str, limit: i64) -> Result<Vec<ConsoleUser>, DataError>;
    async fn get_latest_user(&self, user_id: Uuid) -> Result<ConsoleUser, DataError>;
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
                                       is_blocked,
                                       created_at,
                                       version
                                       )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            user.id,
            user.user_id,
            user.email,
            user.full_name,
            user.allowed_storage,
            user.taken_storage,
            user.is_blocked,
            user.created_at,
            user.version
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_latest(&self, limit: i64, blocked: Option<bool>) -> Result<Vec<ConsoleUser>, DataError> {
        let users = sqlx::query_as!(
            ConsoleUser,
            r#"
            SELECT
                latest.id, latest.user_id,
                latest.email, latest.full_name,
                latest.allowed_storage, latest.taken_storage,
                latest.created_at, latest.updated_at, latest.version
            FROM (
                SELECT DISTINCT ON (user_id) id, user_id, email,
                        full_name, allowed_storage, taken_storage,
                        created_at, updated_at, version
                FROM console_users
                WHERE ($1::bool IS NULL OR is_blocked $1)
                ORDER BY user_id, version DESC
            ) latest
            ORDER BY latest.updated_at DESC
            LIMIT $2

            "#,
            blocked,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }

    async fn get_log(&self, limit: i64, blocked: Option<bool>) -> Result<Vec<ConsoleUser>, DataError> {
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
                is_blocked,
                created_at,
                updated_at,
                version
            FROM console_users
            WHERE ($1::bool IS NULL OR is_blocked $1)
            ORDER BY updated_at DESC
            LIMIT $2
            "#,
            blocked,
            limit
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }

    async fn find_by_query(&self, query: &str) -> Result<Vec<ConsoleUser>, DataError> {
        let users = sqlx::query_as!(
            ConsoleUser,
            r#"
            SELECT
                latest.id, latest.user_id,
                latest.email, latest.full_name,
                latest.allowed_storage, latest.taken_storage, latest.is_blocked,
                latest.created_at, latest.updated_at, latest.version
            FROM (
                SELECT DISTINCT ON (user_id) id, user_id, email,
                        full_name, allowed_storage, taken_storage,
                        is_blocked, created_at, updated_at, version
                FROM console_users
                WHERE user_id::text LIKE lower($1) || '%' OR email ILIKE '%' || $1 || '%'
                ORDER BY user_id, version DESC
            ) latest
            ORDER BY latest.updated_at DESC
            "#,
            query
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(users)
    }

    async fn get_version(&self, query: &str, limit: i64) -> Result<Vec<ConsoleUser>, DataError> {
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
                is_blocked,
                created_at,
                updated_at,
                version
            FROM console_users
            WHERE user_id::text LIKE lower($1) || '%' OR email ILIKE '%' || $1 || '%'
            ORDER BY version DESC
            LIMIT $2
            "#,
            query,
            limit
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
                is_blocked,
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
}
