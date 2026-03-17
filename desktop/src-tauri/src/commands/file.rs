use crate::common::EntityId;
use crate::helpers::mappings::map_file_proto_to_view;
use crate::nas::file_chunk::Data;
use crate::nas::file_service_client::FileServiceClient;
use crate::nas::{FileChunk, InitFileRequest};
use crate::types::model::FileView;
use crate::AppState;
use async_stream::stream;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tonic::Request;

#[tauri::command]
pub async fn init_file(
    state: tauri::State<'_, AppState>,
    name: String,
    destination: String,
    owner_id: String,
    local_path: String,
    is_global: bool,
) -> Result<FileView, String> {
    let metadata = fs::metadata(&local_path)
        .await
        .map_err(|e| format!("Failed to read file metadata for size: {}", e))?;

    let size = metadata.len() as i64;

    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(InitFileRequest {
        name,
        destination: Some(EntityId { value: destination }),
        owner_id: Some(EntityId { value: owner_id }),
        size,
        is_global,
    });

    let response = client
        .init_file(request)
        .await
        .map_err(|e| format!("gRPC InitFile failed: {}", e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn upload_content(
    state: tauri::State<'_, AppState>,
    file_id: String,
    local_path: String,
) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let mut file = File::open(&local_path)
        .await
        .map_err(|e| format!("open file failed: {}", e))?;

    let outbound_stream = stream! {
        yield FileChunk {
            data: Some(Data::FileId(EntityId { value: file_id.clone() })),
        };

        let mut buffer = vec![0; 64 * 1024];

        loop {
            match file.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    yield FileChunk {
                        data: Some(Data::Content(buffer[..n].to_vec())),
                    };
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    break;
                }
            }
        }
    };

    let request = Request::new(outbound_stream);

    client
        .upload_content(request)
        .await
        .map_err(|e| format!("Upload stream failed: {}", e))?;

    Ok(())
}
