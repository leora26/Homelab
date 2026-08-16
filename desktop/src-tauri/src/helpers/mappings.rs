use crate::nas::{
    FileLabelResponse, FileResponse, FileType, FolderResponse, GlobalFileResponse, LabelResponse,
    UploadStatus,
};
use crate::types::model::{FileLabelView, FileView, FolderView, GlobalFileView, LabelView};
use crate::utils::to_unix;

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

        ttl: to_unix(f.ttl),
        created_at: to_unix(f.created_at),
        updated_at: to_unix(f.updated_at),
        deleted_at: to_unix(f.deleted_at),

        // Labels embedded on a file carry no count of their own.
        labels: f
            .labels
            .into_iter()
            .map(|l| map_label_proto_to_view(l, 0))
            .collect(),
    }
}

pub fn map_global_file_proto_to_view(g: GlobalFileResponse) -> GlobalFileView {
    let file = g.file.map(map_file_proto_to_view).unwrap_or_else(|| FileView {
        id: g.original_id.clone().map(|i| i.value).unwrap_or_default(),
        name: String::new(),
        owner_id: String::new(),
        parent_folder_id: String::new(),
        file_type: "Unknown".to_string(),
        is_deleted: false,
        ttl: None,
        size: 0,
        upload_status: "Unknown".to_string(),
        created_at: None,
        updated_at: None,
        deleted_at: None,
        labels: Vec::new(),
    });

    GlobalFileView {
        id: g.id.map(|i| i.value).unwrap_or_default(),
        original_id: g.original_id.map(|i| i.value).unwrap_or_default(),
        file,
        owner_name: g.owner_name,
        shared_at: to_unix(g.shared_at),
    }
}

/// `file_count` is carried alongside the label rather than inside it on the wire, so it
/// is passed in — zero where the source RPC doesn't compute one.
pub fn map_label_proto_to_view(l: LabelResponse, file_count: i64) -> LabelView {
    LabelView {
        id: l.id.map(|i| i.value).unwrap_or_default(),
        name: l.name,
        color: l.color,
        file_count,
        created_at: to_unix(l.created_at),
    }
}

pub fn map_file_label_proto_to_view(fl: FileLabelResponse) -> FileLabelView {
    FileLabelView {
        file_id: fl.file_id.map(|i| i.value).unwrap_or_default(),
        label_id: fl.label_id.map(|i| i.value).unwrap_or_default(),
    }
}

pub fn map_folder_proto_to_view(f: FolderResponse) -> FolderView {
    FolderView {
        id: f.id.map(|i| i.value).unwrap_or_default(),
        parent_folder_id: f.parent_folder_id.map(|id| id.value),
        name: f.name,
        owner_id: f.owner_id.map(|i| i.value).unwrap_or_default(),
        created_at: to_unix(f.created_at),
        deleted_at: to_unix(f.deleted_at),
    }
}
