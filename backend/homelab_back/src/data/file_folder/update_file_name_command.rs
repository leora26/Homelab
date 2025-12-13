use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Deserialize, new)]
pub struct UpdateFileNameCommand {
    pub new_name: String,
}