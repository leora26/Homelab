use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfirmUserCommand {
    pub allowed_storage: i64,
}

impl ConfirmUserCommand {
    pub fn new(allowed_storage: i64) -> Self {
        Self { allowed_storage }
    }
}