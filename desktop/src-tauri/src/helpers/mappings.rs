use crate::nas::{FileResponse, FileType, UploadStatus};
use crate::types::model::FileView;
use crate::utils::format_timestamp;

pub fn map_file_proto_to_view(f: FileResponse) -> FileView {
    let file_type_str = match FileType::try_from(f.file_type) {
        Ok(FileType::Text) => "Text",
        Ok(FileType::Image) => "Image",
        Ok(FileType::Video) => "Video",
        Ok(FileType::Audio) => "Audio",
        Ok(FileType::Pdf) => "PDF",
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
