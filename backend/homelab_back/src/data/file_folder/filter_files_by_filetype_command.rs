use serde::Deserialize;
use crate::domain::file::FileType;

#[derive(Debug, Deserialize)]
pub struct FilterFilesByFileTypeCommand {
    pub file_types: Vec<FileType>
}