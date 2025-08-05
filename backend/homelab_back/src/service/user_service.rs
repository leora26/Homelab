use sqlx::PgPool;
use crate::data::create_user_command::CreateUserCommand;
use crate::domain::user::User;
use crate::db::user_repository;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    user_repository::get_by_email(pool, email).await
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::get_all_users(pool).await
}

pub async fn create_user (pool: &PgPool, data: &CreateUserCommand) -> Result<User, sqlx::Error> {
    user_repository::create_user(pool, data).await
}