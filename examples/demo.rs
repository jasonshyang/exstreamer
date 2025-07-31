use exstreamer::{
    exchanges::{binance::BinanceConfig, bybit::BybitConfig},
    exstreamer::Exstreamer,
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let binance_config = BinanceConfig {
        symbol: "btcusdt".to_string(),
    };

    let bybit_config = BybitConfig {
        depth: 50,
        symbol: "BTCUSDT".to_string(),
    };

    let mut binance_streamer = Exstreamer::new_binance(binance_config, 1);
    let mut binance_stream = binance_streamer.connect().await.unwrap();
    let mut bybit_streamer = Exstreamer::new_bybit(bybit_config, 1);
    let mut bybit_stream = bybit_streamer.connect().await.unwrap();

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

    // Shutdown the streamers
    tokio::try_join!(binance_streamer.shutdown(), bybit_streamer.shutdown())
        .expect("Failed to shutdown streamers");

    tracing::info!("Streamers shut down successfully.");
}
