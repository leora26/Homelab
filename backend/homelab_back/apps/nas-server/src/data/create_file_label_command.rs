use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(new, Debug, Deserialize)]
pub struct CreateFileLabelCommand {
    pub file_id: Uuid,
    pub label_id: Uuid
}