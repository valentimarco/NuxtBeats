use serde::Serialize;
use specta::Type;
use std::time::Duration;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Serialize, Type)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IO(String),

    #[error("Mutex poison occurred: {0}")]
    Mutex(String),

    #[error("Failed to execute command: {0}")]
    Command(String),

    #[error("Operation time out after {0:?}")]
    Timeout(Duration),

    #[error("A tauri error occurred: {0}")]
    Tauri(String),

    #[error("A rustypipe error occurred: {0}")]
    RustyPipe(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::Tauri(err.to_string())
    }
}

impl From<rustypipe::error::Error> for Error {
    fn from(err: rustypipe::error::Error) -> Self {
        Error::RustyPipe(err.to_string())
    }
}
