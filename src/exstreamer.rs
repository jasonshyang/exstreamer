use tokio_util::sync::CancellationToken;

use crate::{
    exchanges::{
        binance::{BinanceConfig, BinanceSubscription},
        bybit::{BybitConfig, BybitSubscription},
    },
    transport::WsClient,
    types::{WsError, WsMsgStream, WsSubscription},
};

pub struct Exstreamer<S: WsSubscription> {
    inner: WsClient<S>,
    shutdown: CancellationToken,
    writer_task: Option<tokio::task::JoinHandle<()>>,
    connection_task: Option<tokio::task::JoinHandle<()>>,
}

impl<S: WsSubscription> Exstreamer<S> {
    pub fn new(config: S::Config, subscription_id: u64) -> Self {
        let subscription = S::new(config, subscription_id);
        let shutdown = tokio_util::sync::CancellationToken::new();
        let inner = WsClient::new_with_shutdown(subscription, shutdown.clone());
        Self {
            inner,
            shutdown,
            writer_task: None,
            connection_task: None,
        }
    }

    pub async fn connect(&mut self) -> Result<WsMsgStream<S::Message>, WsError> {
        let result = self.inner.connect().await?;
        self.writer_task = Some(result.writer_task);
        self.connection_task = Some(result.connection_task);
        Ok(result.stream)
    }

    pub async fn shutdown(&mut self) -> Result<(), WsError> {
        self.shutdown.cancel();

        let writer = self
            .writer_task
            .take()
            .ok_or(WsError::ShutdownBeforeConnection)?;
        let connection = self
            .connection_task
            .take()
            .ok_or(WsError::ShutdownBeforeConnection)?;
        tokio::try_join!(writer, connection)?;
        Ok(())
    }

    pub fn shutdown_sync(&self) {
        self.shutdown.cancel();
    }
}

impl Exstreamer<BinanceSubscription> {
    pub fn new_binance(config: BinanceConfig, subscription_id: u64) -> Self {
        Exstreamer::new(config, subscription_id)
    }
}

impl Exstreamer<BybitSubscription> {
    pub fn new_bybit(config: BybitConfig, subscription_id: u64) -> Self {
        Exstreamer::new(config, subscription_id)
    }
}
