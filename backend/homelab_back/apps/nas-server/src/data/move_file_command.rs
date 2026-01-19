use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, new)]
pub struct MoveFileCommand {
    pub folder_id: Uuid,
    pub file_id: Uuid,
}
