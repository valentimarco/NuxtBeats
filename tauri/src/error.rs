use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::Duration;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Serialize, Type)]
pub enum Error {
    #[error("An error occurred while running the runtime: {0}")]
    Runtime(String),

    #[error("An IO error occurred: {0}")]
    IO(String),

    #[error("Mutex poison occurred: {0}")]
    Mutex(String),

    #[error("Failed to execute command: {0}")]
    Command(String),

    #[error("Operation time out after {0:?}")]
    Timeout(Duration),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::Runtime(err.to_string())
    }
}
