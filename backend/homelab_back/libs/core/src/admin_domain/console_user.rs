use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, new)]
pub struct ConsoleUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub allowed_storage: i64,
    pub taken_storage: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub version: i16
}