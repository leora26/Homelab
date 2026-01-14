use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, new)]
pub struct StorageProfile {
    pub user_id: Uuid,
    pub allowed_storage: i64,
    pub taken_storage: i64,
}

impl StorageProfile {
    pub fn validate_storage_size(&self, file_size: i64) -> bool {
        let future_size = self.taken_storage + file_size;

        if future_size > self.allowed_storage {
            false
        } else {
            true
        }
    }
}
