use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, new)]
pub struct GlobalFile {
    pub id: Uuid,
    pub original_id: Uuid,
    pub created_at: OffsetDateTime
}
