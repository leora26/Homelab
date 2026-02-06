use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::file::{FileType, UploadStatus};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, new)]
pub struct ConsoleFile {
    pub id: Uuid,
    pub file_id: Uuid,
    pub file_type: FileType,
    pub is_deleted: bool,
    pub ttl: Option<OffsetDateTime>,
    pub size: i64,
    pub upload_status: UploadStatus,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub version: i16
}