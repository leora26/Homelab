use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SearchFilesCommand {
    pub owner_id: Uuid,
    pub name: Option<String>,
    pub label_ids: Vec<Uuid>,
    pub updated_after: Option<OffsetDateTime>,
    pub updated_before: Option<OffsetDateTime>,
}

