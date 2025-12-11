use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWhiteListedUserCommand {
    pub email: String,
    pub full_name: String
}

impl CreateWhiteListedUserCommand {
    pub fn new(email: String, full_name: String) -> Self {
        Self {email, full_name}
    }
}