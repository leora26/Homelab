use uuid::Uuid;

pub enum FileType {
    Text,
    Image,
    Video,
    Unknown
}

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub file_type: FileType
}