use crate::data::create_shared_file_command::CreateSharedFileCommand;
use crate::helpers::error_mapping::map_data_err_to_http;
use crate::AppState;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, post, HttpResponse, Responder};
use uuid::Uuid;

#[post("/shared")]
pub async fn create_shared_file_record(
    app_state: Data<AppState>,
    req: Json<CreateSharedFileCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let user_id = &command.user_id.clone();

    match app_state
        .shared_file_service
        .create_shared_file(command)
        .await
    {
        Ok(sh) => HttpResponse::Created().json(sh),
        Err(e) => {
            tracing::error!("Failed to share with user: {}", user_id);
            map_data_err_to_http(e)
        }
    }
}

#[get("/shared")]
pub async fn get_all(app_state: Data<AppState>, user_id: Path<Uuid>) -> impl Responder {
    match app_state
        .shared_file_service
        .get_all_shared_files_per_user(user_id.into_inner())
        .await
    {
        Ok(sf) => {
            if sf.is_empty() {
                HttpResponse::Ok().body("No shared files were found")
            } else {
                HttpResponse::Ok().json(sf)
            }
        }
        Err(e) => {
            tracing::error!("Failed while retrieving files for user");
            map_data_err_to_http(e)
        }
    }
}

pub fn config(c: &mut ServiceConfig) {
    c.service(create_shared_file_record);
    c.service(get_all);
}
