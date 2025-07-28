use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub enum Role {
    User,
    Admin
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    pub role: Role
}