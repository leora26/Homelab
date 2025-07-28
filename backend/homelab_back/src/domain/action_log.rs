use time::OffsetDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
pub enum ActionLogType {
    FileUpload,
    FileDeletion,
    FolderCreation,
    FolderDeletion,
    UserCreation,
    AccountCompletion
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionLog {
    pub id: Uuid,
    pub user_email: String,
    pub log_type: ActionLogType,
    pub file_id: Uuid,
    pub folder_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}