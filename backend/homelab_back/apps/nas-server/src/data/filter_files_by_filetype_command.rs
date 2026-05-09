use homelab_core::nas_domain::file::FileType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterFilesByFileTypeCommand {
    pub file_types: Vec<FileType>,
}
