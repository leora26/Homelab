use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, new)]
pub struct CopyFileCommand {
    pub file_id: Uuid,
    pub target_folder_id: Uuid,
    pub user_id: Uuid,
}
