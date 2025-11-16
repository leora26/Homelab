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
    pub created_at: OffsetDateTime,
}

impl Folder {
    pub fn new_root(id: Uuid, owner_id: Uuid, owner_email: String) -> Self {
        Self {
            id,
            parent_folder_id: None,
            name: owner_email,
            owner_id,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn new(id: Uuid, parent_folder_id: Option<Uuid>, name: String, owner_id: Uuid) -> Self {
        Self {
            id,
            parent_folder_id,
            name,
            owner_id,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}