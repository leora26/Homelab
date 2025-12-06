use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfirmUserCommand {
    pub allowed_storage: i64
}