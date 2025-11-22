use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use crate::AppState;
use tracing;
use crate::data::create_user_command::CreateUserCommand;
use crate::exception::data_error::DataError::DatabaseError;
use crate::helpers::error_mapping::map_data_err_to_http;

#[get("/users/{email}")]
pub async fn get_user_by_email(
    app_state: Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let email = path.into_inner();

    match app_state.user_service.get_by_email(&email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user was found with email {}", email)),
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


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_email);
    cfg.service(get_users);
    cfg.service(create_user);
}

