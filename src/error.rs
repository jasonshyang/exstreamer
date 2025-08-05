use std::fmt::Debug;

use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Error, Debug)]
pub enum ExStreamError {
    #[error("Empty subscription list")]
    EmptySubscriptionList,
    #[error("Unsupported message error: {0}")]
    UnsupportedMessage(String),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Tungstenite error: {0}")]
    TungsteniteError(#[from] Box<tungstenite::Error>),
    #[error("Task error: {0}")]
    TaskError(#[from] tokio::task::JoinError),
    #[error("Handler error: sending a message after the stream is closed")]
    StreamClosed,
}

impl From<tokio_tungstenite::tungstenite::Error> for ExStreamError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        ExStreamError::from(Box::new(error))
    }
}
