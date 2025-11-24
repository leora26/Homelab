use serde::Deserialize;
use uuid::Uuid;
use crate::data::file_folder::file_data::FileData;

#[derive(Debug, Deserialize)]
pub struct UploadFileCommand {
    pub destination_folder_id: Uuid,
    pub file: FileData,
    pub owner_id: Uuid,
}

impl UploadFileCommand {
    pub fn new(destination_folder_id: Uuid, file: FileData, owner_id: Uuid) -> UploadFileCommand {
        UploadFileCommand {
            destination_folder_id,
            file,
            owner_id,
        }
    }
}