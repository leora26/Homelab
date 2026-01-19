use derive_new::new;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub trait DomainEvent {
    fn routing_key (&self) -> &'static str;
}

#[derive(Deserialize, Serialize, Debug, Clone, new)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: OffsetDateTime,
    pub default_storage: i64
}

impl DomainEvent for UserCreatedEvent {
    fn routing_key (&self) -> &'static str {
        "user.created"
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, new)]
pub struct WhiteListedUserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: OffsetDateTime
}

impl DomainEvent for WhiteListedUserCreatedEvent {
    fn routing_key(&self) -> &'static str {
        "whitelisted.user.created"
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, new)]
pub struct UserConfirmedEvent {
    pub user_id: Uuid,
}

impl DomainEvent for UserConfirmedEvent {
    fn routing_key(&self) -> &'static str {
        "whitelisted.user.confirmed"
    }
}