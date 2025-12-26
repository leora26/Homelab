use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, new, FromRow)]
pub struct Label {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub owner_id: Uuid
}