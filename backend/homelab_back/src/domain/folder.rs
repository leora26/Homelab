use uuid::Uuid;

pub struct Folder {
    pub id: Uuid,
    pub is_root: bool,
    pub name: String,
    pub owner_id: Uuid
}