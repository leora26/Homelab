use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Entity {0} was not found")]
    EntityNotFoundException(String),
    #[error("Error while trying to retrieve data from the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Error while creating an Entity: {0}")]
    EntityCreationError(String),
    #[error("Failed to create path to a folder")]
    FolderPathCreationError(),
}