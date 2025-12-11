use derive_new::new;
use serde::Deserialize;

#[derive(Deserialize, Debug, new)]
pub struct UpdatePasswordCommand {
    pub password: String
}