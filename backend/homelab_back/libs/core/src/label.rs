use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, new, FromRow)]
pub struct Label {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub owner_id: Uuid,
}

impl Label {
    pub fn update(&mut self, name: String, color: String) {
        self.name = name;
        self.color = color;
    }
}
