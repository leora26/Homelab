use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Deserialize, new)]
pub struct ConfirmUserCommand {
    pub allowed_storage: i64,
}