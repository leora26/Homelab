use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::user::{Role, User};
use uuid::Uuid;
use crate::exception::data_error::DataError;
use crate::exception::data_error::DataError::DatabaseError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError>;
    async fn get_all(&self) -> Result<Vec<User>, DataError>;
    async fn create(&self, user: User) -> Result<User, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError>;
    async fn save (&self, user: User) -> Result<(), DataError>;
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
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _"
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
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _"
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
        INSERT INTO users (id, email, full_name, password_hash, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, full_name, password_hash, created_at, role as "role: _"
        "#,
        user.id,
        user.email,
        user.full_name,
        user.password_hash,
        user.role as Role
    )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DataError> {
        let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, full_name, password_hash, created_at,  role as "role: _"
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
}