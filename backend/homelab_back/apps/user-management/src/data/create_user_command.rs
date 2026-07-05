use derive_new::new;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, new)]
pub struct CreateUserCommand {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
}
