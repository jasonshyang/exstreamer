# exstreamer

**Exstreamer** is a lightweight, extensible WebSocket client framework for streaming real-time market data from crypto exchanges.

Supported exchanges:
- Bybit: Orderbook, Trade
- Binance: Trade

## Usage

Usage is straightforward, use the `StreamBuilder` to create a stream, which returns a stream and a handler.
```rust
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
```

Use the handler to manage the stream.
```rust
let new_sub = Subscription::new(
    SubscriptionKind::Trade,
    "solusdt",
    None,
    None,
);
binance_handler.subscribe(new_sub).expect("Failed to subscribe to new Binance subscription");
```

## Demo

See [examples/demo.rs](examples/demo.rs) for a full demo.