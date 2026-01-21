use crate::helpers::data_error::DataError;
use tonic::Status;

impl From<DataError> for Status {
    fn from(e: DataError) -> Self {
        tracing::error!("Service Error: {:?}", e);

        match e {
            DataError::EntityNotFoundException(msg) => Status::not_found(msg),
            DataError::DatabaseError(_) => Status::internal("A database error occurred"),
            DataError::WhiteListedUserDoesNotExist(msg) => Status::not_found(msg),
            DataError::UnknownError(msg) => Status::internal(msg),
        }
    }
}
