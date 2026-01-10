use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Deserialize, new)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub storage: i64
}