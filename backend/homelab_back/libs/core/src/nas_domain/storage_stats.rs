use derive_new::new;

#[derive(new, Clone, Debug)]
pub struct StorageStats {
    pub file_count: i64,
    pub folder_count: i64,
    pub trashed_item_count: i64,
    pub trashed_bytes: i64,
    pub labelled_file_count: i64,
    pub unlabelled_file_count: i64,
    pub shared_file_count: i64,
    pub shared_bytes: i64,
}