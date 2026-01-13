use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait DomainEvent {
    fn routing_key (&self) -> &'static str;
}

#[derive(Deserialize, Serialize, Debug, Clone, new)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub default_storage: i64
}

impl DomainEvent for UserCreatedEvent {
    fn routing_key (&self) -> &'static str {
        "user.created"
    }
}