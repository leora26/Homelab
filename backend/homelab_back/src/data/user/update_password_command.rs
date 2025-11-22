use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdatePasswordCommand {
    pub password: String
}