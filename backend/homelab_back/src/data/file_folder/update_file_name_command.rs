use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFileNameCommand {
    pub new_name: String,
}