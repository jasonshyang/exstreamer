use std::fmt::Debug;
use std::pin::Pin;

use futures_util::{SinkExt as _, Stream, StreamExt as _};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TungsteniteMessage};
use tokio_util::sync::CancellationToken;

use crate::{
    error::ExStreamError,
    models::{Subscription, SubscriptionSource},
};

pub type WsMsgStream<M> = Pin<Box<dyn Stream<Item = Result<M, ExStreamError>> + Send + 'static>>;
pub type ConnectionResult<M> = Result<(WsMsgStream<M>, ConnectionHandler), ExStreamError>;

#[derive(Debug)]
/// Connection handlers that handles WebSocket connection lifecycle
pub struct ConnectionHandler {
    source: SubscriptionSource,
    ws_tx: mpsc::UnboundedSender<TungsteniteMessage>,
    writer_task: tokio::task::JoinHandle<()>,
    connection_task: tokio::task::JoinHandle<()>,
    shutdown: CancellationToken,
}

impl ConnectionHandler {
    /// Add a subscription
    pub fn subscribe(&self, subscription: Subscription) -> Result<(), ExStreamError> {
        tracing::info!("Adding subscription: {:?}", subscription);
        self.ws_tx
            .send(TungsteniteMessage::Text(
                subscription.to_subscription_msg(&self.source).into(),
            ))
            .map_err(|_| ExStreamError::StreamClosed)?;
        Ok(())
    }

    /// Remove a subscription
    pub fn unsubscribe(&self, subscription: Subscription) -> Result<(), ExStreamError> {
        tracing::info!("Removing subscription: {:?}", subscription);
        self.ws_tx
            .send(TungsteniteMessage::Text(
                subscription.to_unsubscription_msg(&self.source).into(),
            ))
            .map_err(|_| ExStreamError::StreamClosed)?;
        Ok(())
    }

    /// Send a custom message to the WebSocket
    pub fn send_message(&self, message: TungsteniteMessage) -> Result<(), ExStreamError> {
        tracing::info!("Sending custom message: {:?}", message);
        self.ws_tx
            .send(message)
            .map_err(|_| ExStreamError::StreamClosed)?;
        Ok(())
    }

    /// Gracefully shutdown the connection
    pub async fn shutdown(self) -> Result<(), ExStreamError> {
        tracing::info!("Shutting down connection handler for source: {:?}", self.source);
        self.shutdown.cancel();

        tokio::try_join!(self.writer_task, self.connection_task)
            .map_err(ExStreamError::TaskError)?;
        Ok(())
    }

    /// Synchronously shutdown the connection
    pub fn shutdown_sync(&self) {
        self.shutdown.cancel();
    }

    /// Check if the connection is still alive
    pub fn is_alive(&self) -> bool {
        !self.writer_task.is_finished() && !self.connection_task.is_finished()
    }
}

/// Establish a WebSocket connection with the given source and subscription messages
pub async fn connect_ws<M>(
    source: SubscriptionSource,
    subscription_msg: impl Into<String>,
) -> ConnectionResult<M>
where
    M: DeserializeOwned + Debug + Send + 'static,
{
    let (ws_stream, _) = connect_async(source.endpoint()).await?;
    let (mut write, mut read) = ws_stream.split();

    // Message channels for forwarding messages to/from the WebSocket
    let (outbound_tx, mut outbound_rx) = mpsc::unbounded_channel::<TungsteniteMessage>();
    let (inbound_tx, inbound_rx) = mpsc::unbounded_channel::<Result<M, ExStreamError>>();

    // Create a cancellation token for graceful shutdown
    let shutdown = CancellationToken::new();

    // Send subscription message
    write
        .send(TungsteniteMessage::Text(subscription_msg.into().into()))
        .await?;

    // Spawn writer task
    let shutdown_signal = shutdown.clone();
    let writer_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(message) = outbound_rx.recv() => {
                    tracing::trace!("Sending message: {:?}", message);
                    if write.send(message).await.is_err() {
                        tracing::info!("Failed to send message, WebSocket closed");
                        break;
                    }
                }
                _ = shutdown_signal.cancelled() => {
                    tracing::info!("Shutdown signal received on writer task, terminating.");
                    if write.send(TungsteniteMessage::Close(None)).await.is_err() {
                        tracing::info!("Failed to send close message, WebSocket closed");
                    }
                    break;
                }
            }
        }
        tracing::info!("Writer task finished, no more messages to send.");
    });

    // Spawn connection task
    let shutdown_signal = shutdown.clone();
    let ping_pong_tx = outbound_tx.clone();
    let connection_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                message = read.next() => {
                    match message {
                        Some(Ok(TungsteniteMessage::Text(text))) => {
                            tracing::trace!("Received text message: {}", text);

                            let msg = serde_json::from_str::<M>(&text)
                                .map_err(ExStreamError::SerdeError);
                            tracing::trace!("Parsed message: {:?}", msg);

                            if inbound_tx.send(msg).is_err() {
                                tracing::info!("Failed to send {text}, inbound message channel closed");
                                break;
                            }
                        }
                        Some(Ok(TungsteniteMessage::Ping(ping))) => {
                            tracing::trace!("Received ping: {:?}", ping);

                            if ping_pong_tx.send(TungsteniteMessage::Pong(ping)).is_err() {
                                tracing::info!("Failed to send pong, outbound message channel closed");
                                break;
                            };
                        }
                        Some(Ok(TungsteniteMessage::Pong(pong))) => {
                            tracing::trace!("Received pong: {:?}", pong);
                        }
                        Some(Ok(TungsteniteMessage::Close(_))) => {
                            tracing::info!("WebSocket connection closed");

                            break;
                        }
                        Some(Ok(msg)) => {
                            tracing::warn!("Received unsupported message type");

                            if inbound_tx.send(Err(ExStreamError::UnsupportedMessage(msg.to_string()))).is_err() {
                                tracing::info!("Failed to forward unsupported message, inbound message channel closed");
                                break;
                            }
                            continue;
                        }
                        Some(Err(e)) => {
                            tracing::error!("Error receiving message: {:?}", e);

                            if inbound_tx.send(Err(ExStreamError::TungsteniteError(Box::new(e)))).is_err() {
                                tracing::info!("Failed to forward error, inbound message channel closed");
                            }
                            break;
                        }
                        None => {
                            tracing::info!("WebSocket client closed by server.");
                            break;
                        }
                    }
                }
                _ = shutdown_signal.cancelled() => {
                    tracing::info!("Shutdown signal received on connection task, terminating.");
                    break;
                }
            }
        }
    });

    let handler = ConnectionHandler {
        source,
        ws_tx: outbound_tx,
        writer_task,
        connection_task,
        shutdown,
    };

    let stream = Box::pin(UnboundedReceiverStream::new(inbound_rx));

    Ok((stream, handler))
}
