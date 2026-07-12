pub mod nas {
    tonic::include_proto!("nas");
}

pub mod common {
    tonic::include_proto!("common");
}

pub mod user {
    tonic::include_proto!("user");
}

pub mod commands;
pub mod helpers;
pub mod types;
pub mod utils;

use tauri::Listener;
use tauri::Emitter;
use tauri::Manager;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use url::Url;

pub struct AppState {
    pub user_grpc_channel: Channel,
    pub nas_grpc_channel: Channel,
    pub access_token: RwLock<Option<String>>,
    pub pkce_verifier: RwLock<Option<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let user_endpoint = Endpoint::from_static("http://[::1]:50052");
    let user_channel = user_endpoint.connect_lazy();

    let nas_endpoint = Endpoint::from_static("http://[::1]:50051");
    let nas_channel = nas_endpoint.connect_lazy();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            println!("A new instance tried to open with args: {:?}", argv);

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }

            if let Some(url_str) = argv.iter().find(|arg| arg.starts_with("pavuk://")) {
                let _ = app.emit("deep-link://new-url", url_str.clone());
            }
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Inside lib.rs setup() closure:
            app.listen("deep-link://new-url", move |event| {
                if let Ok(url_str) = serde_json::from_str::<String>(&event.payload().to_string()) {
                    if let Ok(parsed_url) = Url::parse(&url_str) {
                        if parsed_url.host_str() == Some("callback") || parsed_url.path() == "/callback" {
                            let code = parsed_url
                                .query_pairs()
                                .find(|(key, _)| key == "code")
                                .map(|(_, value)| value.into_owned());

                            if let Some(auth_code) = code {
                                println!("Extracted Auth Code. Exchanging for token...");

                                let app_handle_clone = app_handle.clone();

                                // Spawn a background task to handle the network I/O
                                tauri::async_runtime::spawn(async move {
                                    let state = app_handle_clone.state::<AppState>();

                                    // Extract the verifier we saved during trigger_login
                                    let verifier = {
                                        let lock = state.pkce_verifier.read().await;
                                        lock.clone()
                                    };

                                    if let Some(code_verifier) = verifier {
                                        let client = reqwest::Client::new();
                                        let res = client.post("http://localhost:8085/oauth/v2/token")
                                            .form(&[
                                                ("grant_type", "authorization_code"),
                                                ("client_id", "376372459773100036"),
                                                ("code", &auth_code),
                                                ("redirect_uri", "pavuk://callback"),
                                                ("code_verifier", &code_verifier),
                                            ])
                                            .send()
                                            .await;

                                        match res {
                                            Ok(response) if response.status().is_success() => {
                                                if let Ok(json) = response.json::<serde_json::Value>().await {
                                                    if let Some(access_token) = json["access_token"].as_str() {
                                                        // Lock and store the valid JWT
                                                        let mut token_lock = state.access_token.write().await;
                                                        *token_lock = Some(access_token.to_string());

                                                        // Notify the UI to route to the profile
                                                        app_handle_clone.emit("auth_state_changed", true).unwrap();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("Token exchange failed: {}", e);
                                                app_handle_clone.emit("auth_state_changed", false).unwrap();
                                            }
                                            Ok(failed_res) => {
                                                println!("Token exchange rejected: {:?}", failed_res.text().await);
                                                app_handle_clone.emit("auth_state_changed", false).unwrap();
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            });
            Ok(())
        })
        .manage(AppState {
            user_grpc_channel: user_channel,
            nas_grpc_channel: nas_channel,
            access_token: RwLock::new(None),
            pkce_verifier: RwLock::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::login::trigger_login,
            commands::login::get_auth_status,
            commands::login::logout,
            commands::user::get_user_profile,
            commands::storage_profile::get_storage_profile,
            commands::folder::get_root_folder,
            commands::folder::get_files_for_folder,
            commands::folder::get_subfolders,
            commands::folder::create_folder,
            commands::folder::delete_selected_folder,
            commands::folder::rename_folder,
            commands::folder::cleanup_deleted_folder,
            commands::folder::cleanup_trash,
            commands::folder::get_trash_files_by_folder,
            commands::folder::get_trash_subfolders_by_folder,
            commands::folder::get_deleted_folder,
            commands::folder::restore_folder,
            commands::file::init_file,
            commands::file::upload_content,
            commands::file::rename_file,
            commands::file::delete_file,
            commands::file::get_deleted_files,
            commands::file::restore_file,
            commands::file::delete_chosen_file,
            commands::file::remove_deleted_file,
            commands::file::move_file,
            commands::file::copy_file,
            commands::file::archive_file,
            commands::file::unarchive_file,
            commands::file::search_files,
            commands::download::download_file,
            commands::download::get_file_preview,
            commands::global_file::get_global_files,
            commands::global_file::make_file_global,
            commands::global_file::make_file_private,
            commands::global_file::is_file_global,
            commands::label::get_labels,
            commands::label::create_label,
            commands::label::change_label,
            commands::label::delete_label,
            commands::file_label::create_fl,
            commands::file_label::delete_fl,
            commands::file_label::get_labels_for_file,
            commands::file_label::get_file_for_labels,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}