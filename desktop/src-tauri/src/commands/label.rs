use crate::common::EntityId;
use crate::helpers::auth_token::auth_token;
use crate::helpers::mappings::map_label_proto_to_view;
use crate::helpers::with_auth::with_auth;
use crate::nas::label_service_client::LabelServiceClient;
use crate::nas::{ChangeLabelRequest, CreateLabelRequest, DeleteLabelRequest};
use crate::types::model::LabelView;
use crate::AppState;
use tonic::Request;

#[tauri::command]
pub async fn get_labels(state: tauri::State<'_, AppState>) -> Result<Vec<LabelView>, String> {
    let token = auth_token(&state).await?;
    let mut client =
        LabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(());

    let response = client
        .get_labels(request)
        .await
        .map_err(|err| format!("Couldn't get labels: {:?}", err))?;

    let label_response = response.into_inner();

    let labels = label_response
        .labels
        .into_iter()
        .map(|l| map_label_proto_to_view(l))
        .collect();

    Ok(labels) 
}

#[tauri::command]
pub async fn create_label(
    state: tauri::State<'_, AppState>,
    name: String,
    color: String,
) -> Result<LabelView, String> {
    let token = auth_token(&state).await?;
    let mut client =
        LabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(CreateLabelRequest { name, color });

    let response = client
        .create_label(request)
        .await
        .map_err(|err| format!("Couldn't create label: {:?}", err))?;

    let label_response = response.into_inner();

    let label = map_label_proto_to_view(label_response);

    Ok(label)
}

#[tauri::command]
pub async fn delete_label(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    let token = auth_token(&state).await?;

    let mut client =
        LabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(DeleteLabelRequest {
        id: Some(EntityId { value: id }),
    });

    client
        .delete_label(request)
        .await
        .map_err(|err| format!("Couldn't delete label: {:?}", err))?;

    Ok(())
}

#[tauri::command]
pub async fn change_label(
    state: tauri::State<'_, AppState>,
    id: String,
    name: String,
    color: String,
) -> Result<LabelView, String> {
    let token = auth_token(&state).await?;
    let mut client =
        LabelServiceClient::with_interceptor(state.nas_grpc_channel.clone(), with_auth(token));

    let request = Request::new(ChangeLabelRequest {
        id: Some(EntityId { value: id }),
        name,
        color,
    });

    let response = client
        .change_label(request)
        .await
        .map_err(|err| format!("Couldn't change label: {:?}", err))?;

    let label_response = response.into_inner();

    Ok(map_label_proto_to_view(label_response))
}
