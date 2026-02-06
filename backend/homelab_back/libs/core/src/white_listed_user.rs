use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, new)]
pub struct WhiteListedUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: OffsetDateTime,
}
