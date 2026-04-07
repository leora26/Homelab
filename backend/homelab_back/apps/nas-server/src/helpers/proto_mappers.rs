use homelab_core::file::{File, FileType as DomainFileType, UploadStatus as DomainUploadStatus};
use homelab_core::file_label::FileLabel;
use homelab_core::folder::Folder;
use homelab_core::global_file::GlobalFile;
use homelab_core::label::Label;
use homelab_proto::common::EntityId;
use homelab_proto::nas::{FileLabelResponse, FileResponse, FileType as ProtoFileType, FolderResponse, GlobalFileResponse, LabelResponse, StorageProfileResponse, UploadStatus as ProtoUploadStatus};
use tonic::Status;
use uuid::Uuid;
use homelab_core::storage_profile::StorageProfile;

pub fn map_storage_profile_to_proto(sp: StorageProfile) -> StorageProfileResponse {
    StorageProfileResponse {
        user_id: Option::from(map_id_to_proto(sp.user_id)),
        allowed_storage: sp.allowed_storage,
        taken_storage: sp.taken_storage,
        is_blocked: sp.is_blocked,
    }
}

pub fn map_file_to_proto(f: File) -> FileResponse {
    FileResponse {
        id: Option::from(map_id_to_proto(f.id)),
        name: f.name,
        owner_id: Option::from(map_id_to_proto(f.owner_id)),
        parent_folder_id: Option::from(map_id_to_proto(f.parent_folder_id)),
        file_type: match f.file_type {
            DomainFileType::Image => ProtoFileType::Image,
            DomainFileType::Text => ProtoFileType::Text,
            DomainFileType::Video => ProtoFileType::Video,
            DomainFileType::Audio => ProtoFileType::Audio,
            DomainFileType::Pdf => ProtoFileType::Pdf,
            DomainFileType::Zip => ProtoFileType::Zip,
            DomainFileType::Unknown => ProtoFileType::Unknown,
        } as i32,
        is_deleted: f.is_deleted,
        ttl: f.ttl.map(|t| prost_types::Timestamp {
            seconds: t.unix_timestamp(),
            nanos: t.nanosecond() as i32,
        }),
        size: f.size,
        upload_status: match f.upload_status {
            DomainUploadStatus::Failed => ProtoUploadStatus::Failed,
            DomainUploadStatus::Completed => ProtoUploadStatus::Completed,
            DomainUploadStatus::Pending => ProtoUploadStatus::Pending,
        } as i32,
        created_at: Some(prost_types::Timestamp {
            seconds: f.created_at.unix_timestamp(),
            nanos: f.created_at.nanosecond() as i32,
        }),
        updated_at: Some(prost_types::Timestamp {
            seconds: f.updated_at.unix_timestamp(),
            nanos: f.updated_at.nanosecond() as i32,
        })
    }
}

pub fn map_global_file_to_proto(g: GlobalFile) -> GlobalFileResponse {
    GlobalFileResponse {
        id: Option::from(map_id_to_proto(g.id)),
        original_id: Option::from(map_id_to_proto(g.original_id)),
    }
}

pub fn map_folder_to_proto(f: Folder) -> FolderResponse {
    FolderResponse {
        id: Option::from(map_id_to_proto(f.id)),
        parent_folder_id: f.parent_folder_id.map(map_id_to_proto),
        name: f.name,
        owner_id: Option::from(map_id_to_proto(f.owner_id)),
        created_at: Some(prost_types::Timestamp {
            seconds: f.created_at.unix_timestamp(),
            nanos: f.created_at.nanosecond() as i32,
        }),
    }
}

pub fn map_label_to_proto(l: Label) -> LabelResponse {
    LabelResponse {
        id: Option::from(map_id_to_proto(l.id)),
        name: l.name,
        color: l.color,
        owner_id: Option::from(map_id_to_proto(l.owner_id)),
    }
}

pub fn map_file_label_to_proto(fl: FileLabel) -> FileLabelResponse {
    FileLabelResponse {
        file_id: Option::from(map_id_to_proto(fl.file_id)),
        label_id: Option::from(map_id_to_proto(fl.label_id)),
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
