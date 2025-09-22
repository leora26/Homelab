use std::path::Path;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub enum FileType {
    Text,
    Image,
    Video,
    Unknown,
}

impl FileType {
    pub fn from_filename(name: &str) -> Self {
        let extension = Path::new(name)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        if let Some(ext) = extension {
            match ext.as_str() {
                "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "rs" => FileType::Text,
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => FileType::Image,
                "mp4" | "mov" | "avi" | "mkv" | "webm" => FileType::Video,
                _ => FileType::Unknown
            }
        } else {
            FileType::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub parent_folder_id: Uuid,
    pub file_type: FileType,
}

impl File {
    pub fn get_file_type(name: &str) -> FileType {
        FileType::from_filename(name)
    }

    pub fn new(id: Uuid, name: String, owner_id: Uuid, parent_folder_id: Uuid) -> File {
        let file_type: FileType = File::get_file_type(&name);

        File {
            id,
            name,
            owner_id,
            parent_folder_id,
            file_type,
        }
    }
}