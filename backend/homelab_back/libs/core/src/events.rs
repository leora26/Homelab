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
pub struct UserUpdatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub allowed_storage: i64,
    pub taken_storage: i64
}

impl DomainEvent for UserUpdatedEvent {
    fn routing_key (&self) -> &'static str {
        "user.updated"
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
pub struct WhiteListedUserUpdated {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub is_confirmed: bool,
}

impl DomainEvent for WhiteListedUserUpdated {
    fn routing_key(&self) -> &'static str {
        "whitelisted.user.updated"
    }
}