use thiserror::Error;
use std::io;
use serde_json::Error as SerdeError;

pub type Result<T> = std::result::Result<T, crate::error::MSMQError>;

#[derive(Debug)]
pub enum MSMQError {
    Custom(String),
    Io(io::Error),
    Serde(SerdeError),
    QueueNotFound(String),
    TransactionIdRequired,
    TransactionNotFound,
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

impl std::fmt::Display for MSMQError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MSMQError::Custom(msg) => write!(f, "Custom error: {}", msg),
            MSMQError::Io(err) => write!(f, "IO error: {}", err),
            MSMQError::Serde(err) => write!(f, "Serde error: {}", err),
            MSMQError::QueueNotFound(name) => write!(f, "Queue '{}' not found", name),
            MSMQError::TransactionIdRequired => write!(f, "Transaction ID required for transactional queue"),
            MSMQError::TransactionNotFound => write!(f, "Transaction not found"),
            MSMQError::CommandFormatError => write!(f, "Invalid command format"),
        }
    }
}

