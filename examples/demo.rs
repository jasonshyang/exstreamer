use exstreamer::{
    StreamBuilder,
    models::{BinanceRequest, BybitRequest, KrakenChannel},
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (mut binance_stream, binance_handler) = StreamBuilder::binance()
        .with_trade("btcusdt")
        .with_trade("ethusdt")
        .connect()
        .await
        .unwrap();

    let (mut bybit_stream, bybit_handler) = StreamBuilder::bybit()
        .with_trade("btcusdt")
        .with_orderbook("ethusdt", 50)
        .connect()
        .await
        .unwrap();

    let (mut coinbase_stream, coinbase_handler) = StreamBuilder::coinbase()
        .with_trade("ETH-BTC")
        .connect()
        .await
        .unwrap();

    let (mut kraken_stream, kraken_handler) = StreamBuilder::kraken(KrakenChannel::Trade)
        .with_symbol("BTC/USD")
        .connect()
        .await
        .unwrap();

    // Add a new subscription dynamically
    let new_sub = BinanceRequest::new_subscribe().with_trade("solusdt");
    binance_handler.subscribe(new_sub).unwrap();

    // Remove a subscription dynamically
    let remove_sub = BybitRequest::new_unsubscribe().with_orderbook("ethusdt", 50);
    bybit_handler.unsubscribe(remove_sub).unwrap();

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
            message = coinbase_stream.next() => {
                if let Some(msg) = message {
                    tracing::info!("Received Coinbase message: {:?}", msg);
                } else {
                    tracing::info!("No more messages to receive.");
                    break;
                }
            }
            message = kraken_stream.next() => {
                if let Some(msg) = message {
                    tracing::info!("Received Kraken message: {:?}", msg);
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
    tokio::try_join!(
        binance_handler.shutdown(),
        bybit_handler.shutdown(),
        coinbase_handler.shutdown(),
        kraken_handler.shutdown(),
    )
    .expect("Failed to shutdown streamers");

    tracing::info!("Streamers shut down gracefully.");
}
