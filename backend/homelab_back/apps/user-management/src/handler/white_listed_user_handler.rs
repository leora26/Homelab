use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use uuid::Uuid;
use crate::AppState;
use crate::data::confirm_user_command::ConfirmUserCommand;
use crate::data::create_white_listed_user_command::CreateWhiteListedUserCommand;
use crate::helpers::error_mapping::map_data_err_to_http;

#[get("/white_listed_users")]
pub async fn get_white_listed_users(
    app_state: Data<AppState>
) -> impl Responder {
    match app_state.white_listed_user_service.get_all().await {
        Ok(users) => {
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[post("/white_listed_users/confirm/{userid}")]
pub async fn confirm_white_listed_user(
    app_state: Data<AppState>,
    user_id: Path<Uuid>,
    command: Json<ConfirmUserCommand>
) -> impl Responder {

    match app_state.white_listed_user_service.confirm(user_id.into_inner(), command.into_inner()).await {
        Ok(u) => {
            HttpResponse::Created().json(u)
        }
        Err(e) => {
            tracing::error!("Failed to confirm white listed user");
            map_data_err_to_http(e)
        }
    }
}

#[post("/white_listed_user")]
pub async fn create_white_listed_user(
    app_state: Data<AppState>,
    req: Json<CreateWhiteListedUserCommand>,
) -> impl Responder {
    let command = req.into_inner();

    match app_state.white_listed_user_service.create(command).await {
        Ok(u) => {
            HttpResponse::Created().json(u)
        }
        Err(e) => {
            tracing::error!("Failed to create white listed user");
            map_data_err_to_http(e)
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_white_listed_users);
    cfg.service(confirm_white_listed_user);
    cfg.service(create_white_listed_user);
}