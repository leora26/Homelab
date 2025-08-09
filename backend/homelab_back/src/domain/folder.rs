use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: Uuid,
    pub parent_folder_id: Option<Uuid>,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: OffsetDateTime
}