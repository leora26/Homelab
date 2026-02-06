use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::admin_domain::console_wlu::ConsoleWhiteListedUser;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait WluRepo: Send + Sync {
    async fn log_wlu(&self, wlu: ConsoleWhiteListedUser) -> Result<(), DataError>;
    async fn get_all_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_confirmed_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_not_confirmed_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
    async fn get_latest_wlu(&self, user_id: Uuid) -> Result<ConsoleWhiteListedUser, DataError>;
    async fn get_all_wlu_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleWhiteListedUser>, DataError>;
}

#[derive(new)]
pub struct WluRepoImpl {
    pool: PgPool,
}

#[async_trait]
impl WluRepo for WluRepoImpl {
    async fn log_wlu(&self, wlu: ConsoleWhiteListedUser) -> Result<(), DataError> {
        sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            INSERT INTO console_wlu (id, user_id, email, full_name, is_confirmed, created_at, updated_at, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            wlu.id,
            wlu.user_id,
            wlu.email,
            wlu.full_name,
            wlu.is_confirmed,
            wlu.created_at,
            wlu.updated_at,
            wlu.version
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_all_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        let wlu = sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                is_confirmed,
                created_at,
                updated_at,
                version
            FROM console_wlu
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(wlu)
    }

    async fn get_confirmed_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        let wlu = sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                is_confirmed,
                created_at,
                updated_at,
                version
            FROM console_wlu
            WHERE is_confirmed = true
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(wlu)
    }

    async fn get_not_confirmed_wlu(&self) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        let wlu = sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                is_confirmed,
                created_at,
                updated_at,
                version
            FROM console_wlu
            WHERE is_confirmed = false
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(wlu)
    }

    async fn get_latest_wlu(&self, user_id: Uuid) -> Result<ConsoleWhiteListedUser, DataError> {
        let wlu = sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                is_confirmed,
                created_at,
                updated_at,
                version
            FROM console_wlu
            WHERE user_id = $1
            ORDER BY version DESC
            LIMIT 1
            "#,
            user_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(wlu)
    }

    async fn get_all_wlu_versions(&self, user_id: Uuid) -> Result<Vec<ConsoleWhiteListedUser>, DataError> {
        let wlu = sqlx::query_as!(
            ConsoleWhiteListedUser,
            r#"
            SELECT
                id,
                user_id,
                email,
                full_name,
                is_confirmed,
                created_at,
                updated_at,
                version
            FROM console_wlu
            WHERE user_id = $1
            ORDER BY version DESC
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(wlu)
    }
}
