use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Deserialize, new)]
pub struct CreateWhiteListedUserCommand {
    pub email: String,
    pub full_name: String
}