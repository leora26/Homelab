use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWhiteListedUserCommand {
    pub email: String,
    pub full_name: String
}