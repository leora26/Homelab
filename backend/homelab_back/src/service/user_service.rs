use sqlx::PgPool;
use uuid::Uuid;
use crate::data::create_user_command::CreateUserCommand;
use crate::domain::user::User;
use crate::db::user_repository;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    user_repository::get_by_email(pool, email).await
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::get_all_users(pool).await
}

pub async fn create_user(pool: &PgPool, command: CreateUserCommand) -> Result<User, sqlx::Error> {
    let u = User::new(Uuid::new_v4(), command.email, command.password, command.role);

    user_repository::create_user(pool, u).await
}

pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    user_repository::get_user_by_id(pool, user_id).await
}