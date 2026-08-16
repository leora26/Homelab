use serde::Serialize;

/*
 * View models handed to the webview.
 *
 * Timestamps are unix seconds rather than pre-formatted strings: the UI sorts columns,
 * renders relative dates and filters by range, none of which work on a display string.
 * `None` means the server had no value.
 */

#[derive(Serialize)]
pub struct FolderView {
    pub id: String,
    pub parent_folder_id: Option<String>,
    pub name: String,
    pub owner_id: String,
    pub created_at: Option<i64>,
    pub deleted_at: Option<i64>,
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
    pub role: String,
    pub created_at: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct FileView {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub parent_folder_id: String,
    pub file_type: String,
    pub is_deleted: bool,
    pub ttl: Option<i64>,
    pub size: i64,
    pub upload_status: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    /// Populated by `get_files_for_folder`; empty from every other command.
    pub labels: Vec<LabelView>,
}

#[derive(Serialize, Debug)]
pub struct GlobalFileView {
    pub id: String,
    pub original_id: String,
    pub file: FileView,
    pub owner_name: String,
    pub shared_at: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct LabelView {
    pub id: String,
    pub name: String,
    pub color: String,
    /// Files carrying this label. Zero when the label is embedded in a `FileView`,
    /// where the backend sends no count.
    pub file_count: i64,
    pub created_at: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct FileLabelView {
    pub file_id: String,
    pub label_id: String,
}

#[derive(Serialize)]
pub struct StorageStatsView {
    pub file_count: i64,
    pub folder_count: i64,
    pub trashed_item_count: i64,
    pub trashed_bytes: i64,
    pub labelled_file_count: i64,
    pub unlabelled_file_count: i64,
    pub shared_file_count: i64,
    pub shared_bytes: i64,
}

#[derive(Serialize)]
pub struct MachineInfoView {
    pub hostname: String,
    pub address: String,
    pub uptime_seconds: i64,
    pub app_version: String,
}

/// Emitted on the `upload_progress` event while `upload_content` streams a file.
#[derive(Serialize, Clone)]
pub struct UploadProgress {
    pub file_id: String,
    pub bytes_sent: i64,
    pub total_bytes: i64,
}
