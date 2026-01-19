use homelab_proto::common::EntityId;
use tonic::Status;
use uuid::Uuid;



pub fn map_id_to_proto(id: Uuid) -> EntityId {
    EntityId {
        value: id.to_string(),
    }
}

pub fn map_entity_id(id: Option<EntityId>) -> Result<Uuid, Status> {
    let entity_id = id.ok_or_else(|| Status::invalid_argument("Missing ID"))?;

    Uuid::parse_str(&entity_id.value).map_err(|_| Status::invalid_argument("Invalid UUID format"))
}
