use serde::Deserialize;
use crate::domain::user::Role;

#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub role: Role,
    pub full_name: String
}