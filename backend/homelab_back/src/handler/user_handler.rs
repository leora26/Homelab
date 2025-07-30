use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use crate::service::user_service;
use tracing;

#[get("/users/{email}")]
pub async fn get_user_by_email (
    app_state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let email = path.into_inner();

    match user_service::get_user_by_email(&app_state.db_pool, &email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user was found with email {}", email)),
        Err(e) => {
            tracing::error!("Failed to fetch user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_email);
}
