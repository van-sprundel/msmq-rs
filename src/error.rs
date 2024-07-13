use serde_json::Error as SerdeError;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, crate::error::MSMQError>;

#[derive(Debug, Error)]
pub enum MSMQError {
    #[error("{0}")]
    Custom(String),
    #[error("Io Error: {0:?}")]
    Io(io::Error),
    #[error("Serde Error: {0:?}")]
    Serde(SerdeError),
    #[error("Couldn't find queue {0}")]
    QueueNotFound(String),
    #[error("Transaction could not be found")]
    TransactionNotFound,
    #[error("Unknown command format")]
    CommandFormatError,
}

impl From<String> for MSMQError {
    fn from(err: String) -> Self {
        MSMQError::Custom(err)
    }
}

impl From<io::Error> for MSMQError {
    fn from(err: io::Error) -> Self {
        MSMQError::Io(err)
    }
}

impl From<SerdeError> for MSMQError {
    fn from(err: SerdeError) -> Self {
        MSMQError::Serde(err)
    }
}
