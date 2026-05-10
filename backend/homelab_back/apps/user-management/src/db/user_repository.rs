use std::error::Error;
use crate::helpers::data_error::DataError;
use crate::helpers::data_error::DataError::DatabaseError;
use async_trait::async_trait;
use homelab_core::user_domain::user::{Role, User};
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::auth::resolver::ExternalIdResolver;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, user: User) -> Result<User, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError>;
    async fn save(&self, user: User) -> Result<(), DataError>;
    async fn toggle_blocked(&self, user: User) -> Result<(), DataError>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExternalIdResolver for UserRepositoryImpl {
    async fn resolve_external_id(&self, external_id: &str) -> Result<String, Box<dyn Error>> {
        let record = sqlx::query!(
            "SELECT id FROM users WHERE external_id = $1",
            external_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(record.id.to_string())
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, full_name, created_at,  role as "role: _", is_blocked, external_id
        FROM users
        WHERE email = $1
        "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<User>, DataError> {
        let users = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, full_name, external_id, created_at,  role as "role: _", is_blocked
        FROM users
        "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError(e))?;

        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User, DataError> {
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (id, email, full_name, external_id, role, is_blocked)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, full_name, external_id, created_at, role as "role: _", is_blocked
        "#,
            user.id,
            user.email,
            user.full_name,
            user.external_id,
            user.role as Role,
            user.is_blocked,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, full_name, external_id, created_at,  role as "role: _", is_blocked
        FROM users
        WHERE id = $1
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }

    async fn save(&self, user: User) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email = $1, full_name = $2, role = $3, external_id = $4
            WHERE id = $5
            "#,
            user.email,
            user.full_name,
            user.role as Role,
            user.external_id,
            user.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn toggle_blocked(&self, user: User) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_blocked = $1
            WHERE id = $2
            "#,
            user.is_blocked,
            user.id
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }
}
