use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InitFileCommand {
    pub destination: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub expected_size: i64,
}
