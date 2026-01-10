use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::constants::MAX_ALLOWED_STORAGE;

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
    pub full_name: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    pub role: Role,
    pub allowed_storage: i64,
    pub taken_storage: i64
}

impl User {
    pub fn new_complete(id: Uuid, email: String, full_name: String, password: String, allowed_storage: i64) -> User {
        User {
            id,
            email,
            full_name,
            //TODO: Implement bcrypt here or something
            password_hash: Some(Self::hash_password(&password)),
            created_at: OffsetDateTime::now_utc(),
            role: Role::User,
            allowed_storage: if allowed_storage == 0 {MAX_ALLOWED_STORAGE} else {allowed_storage},
            taken_storage: 0i64
        }
    }

    pub fn new_pending (id: Uuid, email: String, full_name: String, allowed_storage: i64) -> User {
        User {
            id,
            email,
            full_name,
            password_hash: None,
            created_at: OffsetDateTime::now_utc(),
            role: Role::User,
            allowed_storage,
            taken_storage: 0i64
        }
    }

    pub fn is_active (&self) -> bool {
        self.password_hash.is_some()
    }

    pub fn set_password (&mut self, pass: &str) {
        self.password_hash = Some(Self::hash_password(pass))
    }

    pub fn validate_storage_size (&self, file_size: i64) -> bool {
        let future_size = self.taken_storage + file_size;

        if future_size > self.allowed_storage { false } else {true}
    }

    fn hash_password(password: &str) -> String {
        format!("hashed_{}", password)
    }
}