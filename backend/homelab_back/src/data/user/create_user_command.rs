use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub storage: i64
}