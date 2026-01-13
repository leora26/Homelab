use homelab_core::user::{Role as DomainRole, User};
use homelab_core::white_listed_user::WhiteListedUser;
use homelab_proto::common::EntityId;
use homelab_proto::user::{Role as ProtoRole, UserResponse, WhiteListedUserResponse};
use tonic::Status;
use uuid::Uuid;

pub fn map_wlu_to_proto(u: WhiteListedUser) -> WhiteListedUserResponse {
    WhiteListedUserResponse {
        id: Some(EntityId {
            value: u.id.to_string(),
        }),
        email: u.email,
        full_name: u.full_name,
        created_at: Some(prost_types::Timestamp {
            seconds: u.created_at.unix_timestamp(),
            nanos: u.created_at.nanosecond() as i32,
        }),
    }
}

pub fn map_user_to_proto(u: User) -> UserResponse {
    UserResponse {
        id: Some(EntityId {
            value: u.id.to_string(),
        }),
        email: u.email,
        full_name: u.full_name,
        role: match u.role {
            DomainRole::User => ProtoRole::User,
            DomainRole::Admin => ProtoRole::Admin,
        } as i32,
        created_at: Some(prost_types::Timestamp {
            seconds: u.created_at.unix_timestamp(),
            nanos: u.created_at.nanosecond() as i32,
        }),
    }
}

pub fn map_entity_id(id: Option<EntityId>) -> Result<Uuid, Status> {
    let entity_id = id.ok_or_else(|| Status::invalid_argument("Missing ID"))?;

    Uuid::parse_str(&entity_id.value).map_err(|_| Status::invalid_argument("Invalid UUID format"))
}
