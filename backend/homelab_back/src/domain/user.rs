use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    pub role: Role
}

impl User {
    pub fn new(id: Uuid, email: String, password: String, role: Role) -> User {
        User {
            id,
            email,
            //TODO: Implement bcrypt here or something
            password_hash: Self::hash_password(&password),
            created_at: OffsetDateTime::now_utc(),
            role
        }
    }


    fn hash_password(password: &str) -> String {
        format!("hashed_{}", password)
    }
}