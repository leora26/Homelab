use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "shared_file_access_type")]
pub enum SharedFileAccessType {
    ReadOnly,
    Edit,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SharedFile {
    pub id: Uuid,
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub owner_id: Uuid,
    pub access_type: SharedFileAccessType,
}

impl SharedFile {
    pub fn new(
        id: Uuid,
        file_id: Uuid,
        user_id: Uuid,
        owner_id: Uuid,
        access_type: SharedFileAccessType,
    ) -> Self {
        Self {
            id,
            file_id,
            user_id,
            owner_id,
            access_type,
        }
    }
}
