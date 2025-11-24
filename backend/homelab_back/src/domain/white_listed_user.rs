use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct WhiteListedUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: OffsetDateTime
}

impl WhiteListedUser {
    pub fn new (
        id: Uuid,
        email: String,
        full_name: String
    ) -> WhiteListedUser {
        WhiteListedUser{
            id,
            email,
            full_name,
            created_at: OffsetDateTime::now_utc()
        }
    }
}