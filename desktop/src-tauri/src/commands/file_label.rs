use crate::common::EntityId;
use crate::helpers::auth_token::auth_token;
use crate::helpers::mappings::{
    map_file_label_proto_to_view, map_file_proto_to_view, map_label_proto_to_view,
};
use crate::helpers::with_auth::with_auth;
use crate::nas::file_label_service_client::FileLabelServiceClient;
use crate::nas::{
    CreateFileLabelRequest, DeleteFileLabelRequest, GetFilesForLabelRequest,
    GetLabelsForFileRequest,
};
use crate::types::model::{FileLabelView, FileView, LabelView};
use crate::AppState;
use tonic::Request;

#[tauri::command]
pub async fn create_fl(
    state: tauri::State<'_, AppState>,
    file_id: String,
    label_id: String,
) -> Result<FileLabelView, String> {
    let token = auth_token(&state).await?;
    let mut client =
        FileLabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(CreateFileLabelRequest {
        file_id: Some(EntityId { value: file_id }),
        label_id: Some(EntityId { value: label_id }),
    });

    let response = client
        .create_file_label(request)
        .await
        .map_err(|err| format!("Could not create file label{:?}", err))?;

    let fl_response = response.into_inner();

    Ok(map_file_label_proto_to_view(fl_response))
}

#[tauri::command]
pub async fn delete_fl(
    state: tauri::State<'_, AppState>,
    file_id: String,
    label_id: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client =
        FileLabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(DeleteFileLabelRequest {
        file_id: Some(EntityId { value: file_id }),
        label_id: Some(EntityId { value: label_id }),
    });

    client
        .delete_file_label(request)
        .await
        .map_err(|err| format!("Could not delete file label{:?}", err))?;

    Ok(())
}

#[tauri::command]
pub async fn get_labels_for_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<Vec<LabelView>, String> {
    let token = auth_token(&state).await?;

    let mut client =
        FileLabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(GetLabelsForFileRequest {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .get_labels_for_file(request)
        .await
        .map_err(|err| format!("Could not get labels for {:?}", err))?;

    let labels_response = response.into_inner();

    let labels = labels_response
        .labels
        .into_iter()
        .map(|l| map_label_proto_to_view(l))
        .collect();

    Ok(labels)
}

#[tauri::command]
pub async fn get_file_for_labels(
    state: tauri::State<'_, AppState>,
    label_id: String,
) -> Result<Vec<FileView>, String> {
    let token = auth_token(&state).await?;
    let mut client =
        FileLabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(GetFilesForLabelRequest {
        label_id: Some(EntityId { value: label_id }),
    });

    let response = client
        .get_files_for_label(request)
        .await
        .map_err(|err| format!("Could not get files for {:?}", err))?;

    let file_response = response.into_inner();

    let files = file_response
        .files
        .into_iter()
        .map(|f| map_file_proto_to_view(f))
        .collect();

    Ok(files)
}
