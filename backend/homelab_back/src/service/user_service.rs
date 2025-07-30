use sqlx::PgPool;
use crate::domain::user::User;
use crate::db::user_repository;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    user_repository::get_by_email(pool, email).await
}