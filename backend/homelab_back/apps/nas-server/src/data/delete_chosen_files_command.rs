use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct DeleteChosenFilesCommand {
    pub files_ids: Vec<Uuid>
}
