use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, new)]
pub struct FileLabel {
    pub id: Uuid,
    pub file_id: Uuid,
    pub label_id: Uuid
}