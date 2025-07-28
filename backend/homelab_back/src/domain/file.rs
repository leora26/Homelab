use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub enum FileType {
    Text,
    Image,
    Video,
    Unknown
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub file_type: FileType
}