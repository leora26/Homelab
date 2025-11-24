use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub name: String,
    pub data: Vec<u8>,
}

impl FileData {
    pub fn new(name: String, data: Vec<u8>) -> FileData {
        FileData {
            name,
            data,
        }
    }
}