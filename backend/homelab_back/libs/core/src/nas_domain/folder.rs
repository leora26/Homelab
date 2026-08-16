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
    pub is_deleted: bool,
    /// When the folder was moved to the trash; `None` while it is live. Trash sorts
    /// files and folders together, so both carry the same field.
    pub deleted_at: Option<OffsetDateTime>,
}

impl Folder {
    pub fn new_root(id: Uuid, owner_id: Uuid, owner_email: String) -> Self {
        Self {
            id,
            parent_folder_id: None,
            name: owner_email,
            owner_id,
            created_at: OffsetDateTime::now_utc(),
            is_deleted: false,
            deleted_at: None,
        }
    }

    pub fn new(id: Uuid, parent_folder_id: Option<Uuid>, name: String, owner_id: Uuid) -> Self {
        Self {
            id,
            parent_folder_id,
            name,
            owner_id,
            created_at: OffsetDateTime::now_utc(),
            is_deleted: false,
            deleted_at: None,
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
    
    pub fn update_parent_folder(&mut self, new_parent_folder_id: Uuid) {
        self.parent_folder_id = Option::from(new_parent_folder_id);
    }
}
