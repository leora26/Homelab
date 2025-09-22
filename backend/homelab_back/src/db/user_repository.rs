use sqlx::PgPool;
use crate::data::create_user_command::CreateUserCommand;
use crate::domain::user::{Role, User};
use uuid::Uuid;

pub async fn get_by_email (pool: &PgPool, email: &str)
    -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, created_at, role as \"role: _\" FROM users WHERE email = $1",
        email
    )
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_all_users (pool: &PgPool)
    -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, created_at, role as \"role: _\" FROM users"
    )
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn create_user (pool: &PgPool, data: User)
    -> Result<User, sqlx::Error> {

    let user_id = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING id, email, password_hash, created_at, role as \"role: _\"",
        user.id,
        user.email,
        user.password_hash,
        user.role as Role
    )
        .fetch_one(pool)
        .await?;

    Ok(user)
}



pub async fn get_user_by_id (pool: &PgPool, user_id: &Uuid)
-> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, created_at, role FROM users WHERE id = $1",
        user_id
    )
        .fetch_optional(pool)
        .await?;

    Ok(user)
}