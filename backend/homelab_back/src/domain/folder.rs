use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: Uuid,
    pub is_root: bool,
    pub name: String,
    pub owner_id: Uuid
}