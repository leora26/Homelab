pub mod nas {
    tonic::include_proto!("nas");
}

pub mod common {
    tonic::include_proto!("common");
}

pub mod user {
    tonic::include_proto!("user");
}

use serde::Serialize;
use nas::{FileResponse, FileType};
use common::EntityId;

#[derive(Serialize)]
struct FileView {
    id: String,
    name: String,
    size: i64,
    file_type: String,
}

#[tauri::command]
async fn get_files() -> Result<Vec<FileView>, String> {
    let mock_file_proto = FileResponse {
        id: Some(EntityId { value: "uuid-1234-5678".to_string() }),
        name: "My_First_Rust_Video.mp4".to_string(),
        file_type: FileType::Video.into(),
        size: 1024 * 1024 * 50,
        is_deleted: false,
        ..Default::default()
    };

    let file_view  = FileView {
        id: mock_file_proto.id.unwrap().value,
        name: mock_file_proto.name,
        size: mock_file_proto.size,
        // Convert the i32 Enum to a readable string
        file_type: match FileType::try_from(mock_file_proto.file_type).unwrap_or(FileType::Unknown) {
            FileType::Video => "Video".to_string(),
            FileType::Text => "Text".to_string(),
            _ => "Other".to_string(),
        },
    };

    Ok(vec![file_view])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
