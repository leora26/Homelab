use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    pub role: Role,
    pub is_blocked: bool,
    pub external_id: String,
}

impl User {
    pub fn new_complete(id: Uuid, email: String, full_name: String, external_id: String) -> User {
        User {
            id,
            email,
            full_name,
            created_at: OffsetDateTime::now_utc(),
            role: Role::User,
            is_blocked: false,
            external_id,
        }
    }

    pub fn new_pending(id: Uuid, email: String, full_name: String, external_id: String) -> User {
        User {
            id,
            email,
            full_name,
            created_at: OffsetDateTime::now_utc(),
            role: Role::User,
            is_blocked: false,
            external_id,
        }
    }
    
    pub fn toggle_blocked (&mut self, is_blocked: bool ) {
        self.is_blocked = is_blocked
    }
}
