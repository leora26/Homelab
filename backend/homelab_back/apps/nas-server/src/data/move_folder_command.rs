use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, new)]
pub struct MoveFolderCommand {
    pub target_folder: Uuid,
    pub folder_id: Uuid
}