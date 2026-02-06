use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Entity {0} was not found")]
    EntityNotFoundException(String),
    #[error("Whitelisted user {0} does not exist")]
    WhiteListedUserDoesNotExist(String),
    #[error("Error while trying to retrieve data from the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    UnknownError(String),
}
