use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFolderNameCommand {
    pub new_name: String,
}