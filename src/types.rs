use std::fmt::Debug;
use std::pin::Pin;

use futures_util::Stream;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub type WsMsgStream<M> = Pin<Box<dyn Stream<Item = Result<M, WsError>> + Send + 'static>>;

pub trait WsSubscription: Send + Sync {
    type Config;
    type Message: DeserializeOwned + Debug + Send + Sync + 'static;

    fn new(config: Self::Config, id: u64) -> Self;
    fn name(&self) -> &'static str;
    fn endpoint(&self) -> &'static str;
    fn sub_msg(&self) -> String;
    fn unsub_msg(&mut self) -> String;
}

#[derive(Error, Debug)]
pub enum WsError {
    #[error("Unsupported message error: {0}")]
    UnsupportedMessage(String),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Tungstenite error: {0}")]
    TungsteniteError(#[from] tungstenite::Error),
    #[error("Shutdown received before connection established")]
    ShutdownBeforeConnection,
    #[error("Task error: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}
