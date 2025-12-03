use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json, ServiceConfig};
use crate::AppState;
use crate::data::file_folder::create_shared_file_command::CreateSharedFileCommand;
use crate::helpers::error_mapping::map_data_err_to_http;

#[post("/shared")]
pub async fn create_shared_file_record (
    app_state: Data<AppState>,
    req: Json<CreateSharedFileCommand>
) -> impl Responder {

    let command = req.into_inner();
    let user_id = &command.user_id.clone();

    match app_state.shared_file_service.create_shared_file(command).await {
        Ok(sh) => HttpResponse::Created().json(sh),
        Err(e) => {
            tracing::error!("Failed to share with user: {}", user_id);
            map_data_err_to_http(e)
        }
    }
}


pub fn config(c: &mut ServiceConfig) {
    c.service(create_shared_file_record);
}