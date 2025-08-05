use exstreamer::{
    StreamBuilder,
    models::{Subscription, SubscriptionKind},
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (mut binance_stream, binance_handler) = StreamBuilder::binance()
        .trade("btcusdt".to_string())
        .trade("ethusdt".to_string())
        .connect()
        .await
        .expect("Failed to create Binance streamer");

    let (mut bybit_stream, bybit_handler) = StreamBuilder::bybit()
        .trade("BTCUSDT".to_string())
        .orderbook("ETHUSDT".to_string())
        .connect()
        .await
        .expect("Failed to create Bybit streamer");

    // Add a new subscription dynamically
    let new_sub = Subscription::new(SubscriptionKind::Trade, "solusdt", None, None);
    binance_handler
        .subscribe(new_sub)
        .expect("Failed to subscribe to new Binance subscription");

    // Receive messages
    loop {
        tokio::select! {
            message = binance_stream.next() => {
                if let Some(msg) = message {
                    tracing::info!("Received Binance message: {:?}", msg);
                } else {
                    tracing::info!("No more messages to receive.");
                    break;
                }
            }
            message = bybit_stream.next() => {
                if let Some(msg) = message {
                    tracing::info!("Received Bybit message: {:?}", msg);
                } else {
                    tracing::info!("No more messages to receive.");
                    break;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received Ctrl+C, shutting down...");
                break;
            }
        }
    }

    // Shutdown the connections
    tokio::try_join!(binance_handler.shutdown(), bybit_handler.shutdown())
        .expect("Failed to shutdown streamers");

    tracing::info!("Streamers shut down gracefully.");
}
