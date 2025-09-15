use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub name: String,
    pub content_type: String,
    pub data: Vec<u8>
}