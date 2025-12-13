use tonic::Status;
use uuid::Uuid;
use crate::domain::file::{File, FileType as DomainFileType, UploadStatus as DomainUploadStatus};
use crate::domain::folder::Folder;
use crate::domain::user::{User, Role as DomainRole};
use crate::domain::white_listed_user::WhiteListedUser;
use crate::pb::{EntityId, FileResponse, Role as ProtoRole, FileType as ProtoFileType, UploadStatus as ProtoUploadStatus, UserResponse, WhiteListedUserResponse, FolderResponse};

pub fn map_wlu_to_proto(u: WhiteListedUser) -> WhiteListedUserResponse {
    WhiteListedUserResponse {
        id: Some(EntityId { value: u.id.to_string() }),
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
        id: Some(EntityId { value: u.id.to_string() }),
        email: u.email,
        full_name: u.full_name,
        role: match u.role {
            DomainRole::User => ProtoRole::User,
            DomainRole::Admin => ProtoRole::Admin,
        } as i32,
        allowed_storage: u.allowed_storage,
        taken_storage: u.taken_storage,
        created_at: Some(prost_types::Timestamp {
            seconds: u.created_at.unix_timestamp(),
            nanos: u.created_at.nanosecond() as i32,
        }),
    }
}

pub fn map_file_to_proto (f: File) -> FileResponse {
    FileResponse {
        id: Some(EntityId {value: f.id.to_string()}),
        name: f.name,
        owner_id: Some(EntityId {value: f.owner_id.to_string()}),
        parent_folder_id: Some(EntityId {value: f.parent_folder_id.to_string()}),
        file_type: match f.file_type {
            DomainFileType::Image => ProtoFileType::Image,
            DomainFileType::Text => ProtoFileType::Text,
            DomainFileType::Video => ProtoFileType::Video,
            DomainFileType::Unknown => ProtoFileType::Unknown,
        } as i32,
        is_deleted: f.is_deleted,
        ttl: Some(prost_types::Timestamp {
            seconds: f.ttl.unwrap().unix_timestamp(),
            nanos: f.ttl.unwrap().nanosecond() as i32
        }),
        size: f.size,
        upload_status: match f.upload_status {
            DomainUploadStatus::Failed => ProtoUploadStatus::Failed,
            DomainUploadStatus::Completed => ProtoUploadStatus::Completed,
            DomainUploadStatus::Pending => ProtoUploadStatus::Pending,
        } as i32
    }
}

pub fn map_folder_to_proto (f: Folder) -> FolderResponse {
    FolderResponse {
        id: Option::from(map_id_to_proto(f.id)),
        parent_folder_id: Option::from(map_id_to_proto(f.parent_folder_id.unwrap())),
        name: f.name,
        owner_id: Option::from(map_id_to_proto(f.owner_id)),
        created_at: Some(prost_types::Timestamp {
            seconds: f.created_at.unix_timestamp(),
            nanos: f.created_at.nanosecond() as i32
        })
    }
}

pub fn map_id_to_proto (id: Uuid) -> EntityId {
    EntityId {value: id.to_string()}
}

pub fn map_entity_id (id: Option<EntityId>) -> Result<Uuid, Status> {
    let entity_id = id.ok_or_else(|| Status::invalid_argument("Missing ID"))?;

    Uuid::parse_str(&entity_id.value)
        .map_err(|_| Status::invalid_argument("Invalid UUID format"))
}