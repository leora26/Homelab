use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, new)]
pub struct InitFileCommand {
    pub destination: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub expected_size: i64,
    pub is_global: bool,
}
