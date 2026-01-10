use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, new)]
pub struct ChangeLabelCommand {
    pub id: Uuid,
    pub name: String,
    pub color: String
}