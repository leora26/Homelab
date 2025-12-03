use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateSharedFileCommand {
    pub user_id: Uuid,
    pub owner_id: Uuid,
    pub file_id: Uuid
}