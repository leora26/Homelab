pub mod nas {
    tonic::include_proto!("nas");
}

pub mod common {
    tonic::include_proto!("common");
}

pub mod user {
    tonic::include_proto!("user");
}

pub mod utils;
pub mod commands;
pub mod types;
pub mod helpers;
use tonic::transport::{Channel, Endpoint};

pub struct AppState {
    pub user_grpc_channel: Channel,
    pub nas_grpc_channel: Channel,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let user_endpoint = Endpoint::from_static("http://[::1]:50052");
    let user_channel = user_endpoint.connect_lazy();

    let nas_endpoint = Endpoint::from_static("http://[::1]:50051");
    let nas_channel = nas_endpoint.connect_lazy();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())

        .manage(AppState {
            user_grpc_channel: user_channel,
            nas_grpc_channel: nas_channel,
        })
        .invoke_handler(tauri::generate_handler![
            commands::user::get_user_profile,
            commands::storage_profile::get_storage_profile,
            commands::folder::get_root_folder,
            commands::folder::get_files_for_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
