use actix_web::{get, post, web, HttpResponse, Responder};
use crate::AppState;
use tracing;
use crate::data::create_user_command::CreateUserCommand;
use crate::exception::data_error::DataError::DatabaseError;

#[get("/users/{email}")]
pub async fn get_user_by_email(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let email = path.into_inner();

    match app_state.user_service.get_by_email(&email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user was found with email {}", email)),
        Err(e) => {
            tracing::error!("Failed to fetch user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users")]
pub async fn get_users(
    app_state: web::Data<AppState>
) -> impl Responder {
    match app_state.user_service.get_all().await {
        Ok(users) => {
            if users.is_empty() {
                HttpResponse::NotFound().body("No users were found in the system")
            } else {
                HttpResponse::Ok().json(users)
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/users")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    req: web::Json<CreateUserCommand>,
) -> impl Responder {
    let command = req.into_inner();

    match app_state.user_service.create(command).await {
        Ok(user) => {
            HttpResponse::Created().json(user)
        }
        Err(e) => {
            match e {
                DatabaseError(db_err) => {
                    tracing::error!("Database constraint error: {:?}", db_err);
                    HttpResponse::Conflict().body(format!("Database error: {}", db_err))
                },
                _ => {
                    tracing::error!("Failed to create user: {:?}", e);
                    HttpResponse::InternalServerError().body(format!("{}", e))
                }
            }
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_email);
    cfg.service(get_users);
    cfg.service(create_user);
}

