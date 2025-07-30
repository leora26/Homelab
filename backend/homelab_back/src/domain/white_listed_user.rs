use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct WhiteListedUser {
    pub id: Uuid,
    pub email: String,
    pub created_at: OffsetDateTime
}