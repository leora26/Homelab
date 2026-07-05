use crate::nas::{FileResponse, FileType, FolderResponse, GlobalFileResponse, UploadStatus};
use crate::types::model::{FileView, FolderView, GlobalFileView};
use crate::utils::format_timestamp;

pub fn map_file_proto_to_view(f: FileResponse) -> FileView {
    let file_type_str = match FileType::try_from(f.file_type) {
        Ok(FileType::Text) => "Text",
        Ok(FileType::Image) => "Image",
        Ok(FileType::Video) => "Video",
        Ok(FileType::Audio) => "Audio",
        Ok(FileType::Pdf) => "PDF",
        Ok(FileType::Zip) => "Zip",
        _ => "Unknown",
    }
    .to_string();

    let upload_status_str = match UploadStatus::try_from(f.upload_status) {
        Ok(UploadStatus::Pending) => "Pending",
        Ok(UploadStatus::Completed) => "Completed",
        Ok(UploadStatus::Failed) => "Failed",
        _ => "Unknown",
    }
    .to_string();

    FileView {
        id: f.id.map(|i| i.value).unwrap_or_default(),
        name: f.name,
        owner_id: f.owner_id.map(|i| i.value).unwrap_or_default(),
        parent_folder_id: f.parent_folder_id.map(|i| i.value).unwrap_or_default(),

        file_type: file_type_str,
        upload_status: upload_status_str,

        is_deleted: f.is_deleted,
        size: f.size,

        ttl: f.ttl.map(|ts| format_timestamp(Some(ts))),

        created_at: format_timestamp(f.created_at),
        updated_at: format_timestamp(f.updated_at),
    }
}

pub fn map_global_file_proto_to_view(g: GlobalFileResponse) -> GlobalFileView {
    let file = g
        .file
        .map(map_file_proto_to_view)
        .unwrap_or_else(|| FileView {
            id: g.original_id.clone().map(|i| i.value).unwrap_or_default(),
            name: String::new(),
            owner_id: String::new(),
            parent_folder_id: String::new(),
            file_type: "Unknown".to_string(),
            is_deleted: false,
            ttl: None,
            size: 0,
            upload_status: "Unknown".to_string(),
            created_at: String::new(),
            updated_at: String::new(),
        });

    GlobalFileView {
        id: g.id.map(|i| i.value).unwrap_or_default(),
        original_id: g.original_id.map(|i| i.value).unwrap_or_default(),
        file,
    }
}

pub fn map_folder_proto_to_view(f: FolderResponse) -> FolderView {
    FolderView {
        id: f.id.map(|i| i.value).unwrap_or_default(),
        parent_folder_id: f.parent_folder_id.map(|id| id.value),
        name: f.name,
        owner_id: f.owner_id.map(|i| i.value).unwrap_or_default(),
        created_at: format_timestamp(f.created_at),
    }
}
