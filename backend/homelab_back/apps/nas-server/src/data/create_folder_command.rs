use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct  CreateFolderCommand {
    pub parent_folder_id : Uuid,
    pub name: String,
    pub owner_id: Uuid
}