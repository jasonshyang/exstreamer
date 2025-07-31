use futures_util::{SinkExt as _, StreamExt as _};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::tungstenite;
use tokio_util::sync::CancellationToken;

use crate::types::{WsError, WsMsgStream, WsSubscription};

pub struct WsConnectionResult<M> {
    pub stream: WsMsgStream<M>,
    pub writer_task: tokio::task::JoinHandle<()>,
    pub connection_task: tokio::task::JoinHandle<()>,
}

pub struct WsClient<S> {
    subscription: S,
    shutdown: CancellationToken,
}

impl<S: WsSubscription> WsClient<S> {
    pub fn new(subscription: S) -> Self {
        Self {
            subscription,
            shutdown: CancellationToken::new(),
        }
    }

    pub fn new_with_shutdown(subscription: S, shutdown: CancellationToken) -> Self {
        Self {
            subscription,
            shutdown,
        }
    }

    pub async fn connect(&mut self) -> Result<WsConnectionResult<S::Message>, WsError> {
        let (ws_stream, _) =
            tokio_tungstenite::connect_async(&self.subscription.endpoint().to_string()).await?;
        let (mut write, mut read) = ws_stream.split();

        // Internal channel
        let (outbound_msg_tx, mut outbound_msg_rx) =
            mpsc::unbounded_channel::<tungstenite::Message>();
        let (inbound_msg_tx, inbound_msg_rx) =
            mpsc::unbounded_channel::<Result<S::Message, WsError>>();

        let sub_msg = self.subscription.sub_msg();
        let unsub_msg = self.subscription.unsub_msg();

        // Send subscription message
        write
            .send(tungstenite::Message::Text(sub_msg.into()))
            .await?;

        // Spawn writer task
        let writer_task = tokio::spawn(async move {
            while let Some(msg) = outbound_msg_rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    tracing::error!("Failed to write message: {:?}", e);
                    break;
                }
            }
            tracing::info!("Writer task finished, no more messages to send.");
        });

        // Spawn connection task
        let shutdown = self.shutdown.clone();
        let connection_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    message = read.next() => {
                        match message {
                            Some(Ok(tungstenite::Message::Text(text))) => {
                                tracing::trace!("Received text message: {}", text);

                                let msg = serde_json::from_str::<S::Message>(&text)
                                    .map_err(WsError::SerdeError);
                                tracing::trace!("Parsed message: {:?}", msg);

                                if inbound_msg_tx.send(msg).is_err() {
                                    tracing::info!("Failed to send {text}, inbound message channel closed");
                                    break;
                                }
                            }
                            Some(Ok(tungstenite::Message::Ping(ping))) => {
                                tracing::trace!("Received ping: {:?}", ping);

                                if outbound_msg_tx.send(tungstenite::Message::Pong(ping)).is_err() {
                                    tracing::info!("Failed to send pong, outbound message channel closed");
                                    break;
                                };
                            }
                            Some(Ok(tungstenite::Message::Pong(pong))) => {
                                tracing::trace!("Received pong: {:?}", pong);
                            }
                            Some(Ok(tungstenite::Message::Close(_))) => {
                                tracing::info!("WebSocket connection closed");

                                if outbound_msg_tx.send(tungstenite::Message::Text(unsub_msg.into())).is_err() {
                                    tracing::info!("Failed to send unsub message, outbound message channel closed");
                                }
                                break;
                            }
                            Some(Ok(msg)) => {
                                tracing::warn!("Received unsupported message type");

                                if inbound_msg_tx.send(Err(WsError::UnsupportedMessage(msg.to_string()))).is_err() {
                                    tracing::info!("Failed to forward unsupported message, inbound message channel closed");
                                    break;
                                }
                                continue;
                            }
                            Some(Err(e)) => {
                                tracing::error!("Error receiving message: {:?}", e);

                                if inbound_msg_tx.send(Err(WsError::TungsteniteError(e))).is_err() {
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
                    _ = shutdown.cancelled() => {
                        tracing::info!("Shutdown signal received, stopping connection task.");
                        if outbound_msg_tx.send(tungstenite::Message::Text(unsub_msg.into())).is_err() {
                            tracing::error!("Failed to send unsubscribe message, outbound message channel closed");
                        }
                        break;
                    }
                }
            }
        });

        Ok(WsConnectionResult {
            stream: Box::pin(UnboundedReceiverStream::new(inbound_msg_rx)),
            writer_task,
            connection_task,
        })
    }
}
