use actix_web::{get, patch, post, web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use crate::AppState;
use tracing;
use uuid::Uuid;
use crate::data::user::create_user_command::CreateUserCommand;
use crate::data::user::update_password_command::UpdatePasswordCommand;
use crate::helpers::error_mapping::map_data_err_to_http;

#[get("/users/{email}")]
pub async fn get_user_by_email(
    app_state: Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let email = path.into_inner();

    match app_state.user_service.get_by_email(&email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user was found with email {}", &email)),
        Err(e) => {
            tracing::error!("Failed to fetch user: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/users")]
pub async fn get_users(
    app_state: Data<AppState>
) -> impl Responder {
    match app_state.user_service.get_all().await {
        Ok(users) => {
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[post("/users")]
pub async fn create_user(
    app_state: Data<AppState>,
    req: Json<CreateUserCommand>,
) -> impl Responder {
    let command = req.into_inner();

    match app_state.user_service.create(command).await {
        Ok(user) => {
            HttpResponse::Created().json(user)
        }
        Err(e) => {
            tracing::error!("Failed to create a user: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[patch("/users/{id}/password")]
pub async fn update_password (
    app_state: Data<AppState>,
    id: Path<Uuid>,
    req: Json<UpdatePasswordCommand>
) -> impl Responder {
    let command = req.into_inner();

    let user_id = id.into_inner();
    
    match app_state.user_service.update_password(user_id, &command.password).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to update password for user: {}", user_id);
            map_data_err_to_http(e)
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_email);
    cfg.service(get_users);
    cfg.service(create_user);
    cfg.service(update_password);
}

