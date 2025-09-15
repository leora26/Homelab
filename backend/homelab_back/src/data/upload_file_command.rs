use serde::Deserialize;
use uuid::Uuid;
use crate::data::file_data::FileData;

#[derive(Debug, Deserialize)]
pub struct UploadFileCommand {
    pub destination_folder_id: Uuid,
    pub file: FileData
}