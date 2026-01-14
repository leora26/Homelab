use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct DeleteChosenFoldersCommand {
    pub folder_ids: Vec<Uuid>,
}
