use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Entity {0} was not found")]
    EntityNotFoundException(String),
    #[error("Error while trying to retrieve data from the database")]
    DatabaseError(#[from] sqlx::Error)
}