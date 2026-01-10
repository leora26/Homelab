use serde::Deserialize;
use homelab_core::file::FileType;

#[derive(Debug, Deserialize)]
pub struct FilterFilesByFileTypeCommand {
    pub file_types: Vec<FileType>
}