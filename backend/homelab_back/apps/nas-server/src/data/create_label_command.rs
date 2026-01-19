use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, new)]
pub struct CreateLabelCommand {
    pub name: String,
    pub color: String,
    pub owner_id: Uuid,
}
