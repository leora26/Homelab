use serde::Deserialize;
use crate::domain::user::Role;

#[derive(Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub role: Role
}