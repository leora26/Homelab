use sqlx::PgPool;
use crate::domain::user::User;

pub async fn get_by_email (pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, created_at, role as \"role: _\" FROM users WHERE email = $1",
        email
    )
        .fetch_optional(pool)
        .await?;

    Ok(user)
}