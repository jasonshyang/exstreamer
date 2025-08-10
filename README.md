# exstreamer

**Exstreamer** is a lightweight, extensible WebSocket client framework for streaming real-time market data from crypto exchanges.

The library is still in active development, currently supported exchanges:
- Bybit: Orderbook, Trade
- Binance: Trade
- Coinbase: Trade (Ticker)

## To-dos
- Handle reconnection
- Add more exchanges

## Usage

Usage is straightforward, use the `StreamBuilder` to create a stream, which returns a stream and a handler.
```rust
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
```

Use the handler to manage the stream.
```rust
// Add a new subscription dynamically
let new_sub = BinanceRequest::new_subscribe().with_trade("solusdt");
binance_handler.subscribe(new_sub).unwrap();

// Remove a subscription dynamically
let remove_sub = BybitRequest::new_unsubscribe().with_orderbook("ethusdt", 50);
bybit_handler.unsubscribe(remove_sub).unwrap();

// Shutdown the connection
binance_handler.shutdown()
```

## Demo

See [examples/demo.rs](examples/demo.rs) for a full demo.