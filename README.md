# exstreamer

**Exstreamer** is a lightweight, extensible WebSocket client framework for streaming real-time market data from crypto exchanges.

Supported exchanges:
- Bybit: Orderbook, Trade
- Binance: Trade

## Usage

See [examples/demo.rs](examples/demo.rs) for a full example.

```rust
use exstreamer::{
    exchanges::{binance::BinanceConfig, bybit::BybitConfig},
    exstreamer::Exstreamer,
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let binance_config = BinanceConfig { symbol: "btcusdt".to_string() };
    let bybit_config = BybitConfig { depth: 50, symbol: "BTCUSDT".to_string() };

    let mut binance_streamer = Exstreamer::new_binance(binance_config, 1);
    let mut binance_stream = binance_streamer.connect().await.unwrap();

    let mut bybit_streamer = Exstreamer::new_bybit(bybit_config, 1);
    let mut bybit_stream = bybit_streamer.connect().await.unwrap();

    // Receive messages
    loop {
        tokio::select! {
            message = binance_stream.next() => {
                if let Some(msg) = message {
                    println!("Binance: {:?}", msg);
                }
            }
            message = bybit_stream.next() => {
                if let Some(msg) = message {
                    println!("Bybit: {:?}", msg);
                }
            }
        }
    }
}
```