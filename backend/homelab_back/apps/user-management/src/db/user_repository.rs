use crate::helpers::data_error::DataError;
use crate::helpers::data_error::DataError::DatabaseError;
use async_trait::async_trait;
use homelab_core::user::{Role, User};
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, user: User) -> Result<User, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError>;
    async fn save(&self, user: User) -> Result<(), DataError>;
    async fn toggle_blocked(&self, user: User) -> Result<(), DataError>;
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _", is_blocked
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
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _", is_blocked
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
        INSERT INTO users (id, email, full_name, password_hash, role, is_blocked)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, full_name, password_hash, created_at, role as "role: _", is_blocked
        "#,
            user.id,
            user.email,
            user.full_name,
            user.password_hash,
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
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _", is_blocked
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
            SET email = $1, full_name = $2, role = $3, password_hash = $4
            WHERE id = $5
            "#,
            user.email,
            user.full_name,
            user.role as Role,
            user.password_hash,
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
