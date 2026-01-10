use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}
