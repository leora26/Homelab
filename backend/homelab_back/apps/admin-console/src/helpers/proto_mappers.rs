use homelab_core::admin_domain::console_user::ConsoleUser;
use homelab_proto::admin::{ConsoleUserResponse, ConsoleWluResponse};
use homelab_proto::common::EntityId;
use tonic::Status;
use uuid::Uuid;
use homelab_core::admin_domain::console_wlu::ConsoleWhiteListedUser;

pub fn map_console_user(u: ConsoleUser) -> ConsoleUserResponse {
    ConsoleUserResponse {
        id: Option::from(map_id_to_proto(u.id)),
        user_id: Option::from(map_id_to_proto(u.user_id)),
        email: u.email,
        full_name: u.full_name,
        allowed_storage: u.allowed_storage,
        taken_storage: u.taken_storage,
        created_at: Some(prost_types::Timestamp {
            seconds: u.created_at.unix_timestamp(),
            nanos: u.created_at.nanosecond() as i32,
        }),
        updated_at: Some(prost_types::Timestamp {
            seconds: u.updated_at.unix_timestamp(),
            nanos: u.updated_at.nanosecond() as i32,
        }),
        version: u.version as i32,
    }
}

pub fn map_console_wlu(u: ConsoleWhiteListedUser) -> ConsoleWluResponse {
    ConsoleWluResponse {
        id: Option::from(map_id_to_proto(u.id)),
        user_id: Option::from(map_id_to_proto(u.user_id)),
        email: u.email,
        full_name: u.full_name,
        is_confirmed: u.is_confirmed,
        created_at: Some(prost_types::Timestamp {
            seconds: u.created_at.unix_timestamp(),
            nanos: u.created_at.nanosecond() as i32,
        }),
        updated_at: Some(prost_types::Timestamp {
            seconds: u.updated_at.unix_timestamp(),
            nanos: u.updated_at.nanosecond() as i32,
        }),
        version: u.version as i32,
    }
}

pub fn map_id_to_proto(id: Uuid) -> EntityId {
    EntityId {
        value: id.to_string(),
    }
}

pub fn map_entity_id(id: Option<EntityId>) -> Result<Uuid, Status> {
    let entity_id = id.ok_or_else(|| Status::invalid_argument("Missing ID"))?;

    Uuid::parse_str(&entity_id.value).map_err(|_| Status::invalid_argument("Invalid UUID format"))
}
