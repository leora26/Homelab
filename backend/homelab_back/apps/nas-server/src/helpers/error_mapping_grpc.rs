use tonic::Status;
use crate::helpers::data_error::DataError;

impl From<DataError> for Status {
    fn from(e: DataError) -> Self {
        tracing::error!("Service Error: {:?}", e);

        match e {
            DataError::EntityNotFoundException(msg) => Status::not_found(msg),
            DataError::DatabaseError(_) => Status::internal("A database error occurred"),
            DataError::EntityCreationError(msg) => Status::aborted(msg),
            DataError::ValidationError(msg) => Status::invalid_argument(msg),
            DataError::NoFreeStorageError => Status::invalid_argument("No free space is available"),
            DataError::IOError(msg) => Status::internal(msg),
            DataError::UploadInterrupter(msg) => Status::internal(msg),
            DataError::NotMatchingByteSizeError => Status::internal("The size of the uploaded file does not match to database metadata"),
            DataError::FileAlreadyExistsError => Status::invalid_argument("A file with the same name already exists in the given folder"),
            DataError::FileIsAlreadyArchivedError => Status::invalid_argument("This file has already been archived"),
            DataError::FileIsNotArchivedError => Status::invalid_argument("You cannot unarchive file that is not an archive"),
            DataError::UnknownError(msg) => Status::internal(msg),
            _ => Status::internal("Internal server error"),
        }
    }
}