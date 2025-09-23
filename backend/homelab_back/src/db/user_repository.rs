use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::user::{Role, User};
use uuid::Uuid;
use crate::exception::data_error::DataError;
use crate::exception::data_error::DataError::DatabaseError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, user: User) -> Result<User, DataError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, DataError>;
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, created_at,  role as \"role: _\" FROM users WHERE email = $1",
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
        "SELECT id, email, password_hash, created_at,  role as \"role: _\" FROM users"
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError(e))?;

        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User, DataError> {
        let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password_hash, role) VALUES ($1, $2, $3, $4) \
        RETURNING id, email, password_hash, created_at, role as \"role: _\"",
        user.id,
        user.email,
        user.password_hash,
        user.role as Role
    )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, created_at,  role as \"role: _\" FROM users WHERE id = $1",
        id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(user)
    }
}