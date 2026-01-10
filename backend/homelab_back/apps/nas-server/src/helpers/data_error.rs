use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Entity {0} was not found")]
    EntityNotFoundException(String),
    #[error("Whitelisted user {0} does not exist")]
    WhiteListedUserDoesNotExist(String),
    #[error("Error while trying to retrieve data from the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Error while creating an Entity: {0}")]
    EntityCreationError(String),
    #[error("Failed to validate data")]
    ValidationError(String),
    #[error("Out of available storage")]
    NoFreeStorageError,
    #[error("Failed during I/O operation: {0}")]
    IOError(String),
    #[error("Failed while uploading file: {0}")]
    UploadInterrupter(String),
    #[error("Not matching byte size")]
    NotMatchingByteSizeError,
    #[error("File with this name already exists in this folder. Delete existing file in order to upload new one")]
    FileAlreadyExistsError,
    #[error("{0}")]
    UnknownError(String),
    #[error("This file has been previously archived and it cannot be archived again")]
    FileIsAlreadyArchivedError,
    #[error("This file is not archived, so you cannot not unarchive it")]
    FileIsNotArchivedError,
}