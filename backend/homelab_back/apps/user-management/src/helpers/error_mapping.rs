use crate::helpers::data_error::DataError;
use actix_web::HttpResponse;

pub fn map_data_err_to_http(e: DataError) -> HttpResponse {
    match e {
        DataError::EntityNotFoundException(msg) => HttpResponse::NotFound().body(msg),
        DataError::ValidationError(msg) => HttpResponse::BadRequest().body(msg),
        DataError::EntityCreationError(msg) => HttpResponse::BadRequest().body(msg),
        DataError::WhiteListedUserDoesNotExist(msg) => HttpResponse::Conflict().body(msg),
        DataError::DatabaseError(err) => HttpResponse::InternalServerError().body(err.to_string()),
        _ => {
            tracing::error!("Internal server error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
