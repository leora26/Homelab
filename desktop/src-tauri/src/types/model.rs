use serde::Serialize;

#[derive(Serialize)]
pub struct FolderView {
    pub id: String,
    pub parent_folder_id: Option<String>,
    pub name: String,
    pub owner_id: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct StorageProfileView {
    pub user_id: String,
    pub allowed_storage: i64,
    pub taken_storage: i64,
    pub is_blocked: bool,
}

#[derive(Serialize)]
pub struct UserProfileView {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct FileView {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub parent_folder_id: String,
    pub file_type: String,
    pub is_deleted: bool,
    pub ttl: Option<String>,
    pub size: i64,
    pub upload_status: String,
    pub created_at: String,
    pub updated_at: String,
}
