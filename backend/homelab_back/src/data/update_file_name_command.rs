use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateFileNameCommand {
    pub new_name: String,
s}