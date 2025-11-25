use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().body("Hey, there i am ready to accept requests")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(test_endpoint);
}