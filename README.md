# exstreamer

**Exstreamer** is a lightweight, extensible WebSocket client framework for streaming real-time market data from crypto exchanges.

Supported exchanges:
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
    .trade("btcusdt")
    .trade("ethusdt")
    .connect()
    .await
    .unwrap();

let (mut bybit_stream, bybit_handler) = StreamBuilder::bybit()
    .trade("btcusdt")
    .orderbook("ethusdt", 50)
    .connect()
    .await
    .unwrap();

let (mut coinbase_stream, coinbase_handler) = StreamBuilder::coinbase()
    .trade("ETH-BTC")
    .connect()
    .await
    .unwrap();
```

Use the handler to manage the stream.
```rust
// Add a new subscription dynamically
let new_sub_request = BinanceRequest::create_trade_request(true, "btcusdt", None);
binance_handler.subscribe(new_sub_request).unwrap();

// Remove a subscription dynamically
let remove_sub_request = BybitRequest::create_orderbook_request(false, "ethusdt", 50, None);
bybit_handler.unsubscribe(remove_sub_request).unwrap();

// Shutdown the connection
binance_handler.shutdown()
```

## Demo

See [examples/demo.rs](examples/demo.rs) for a full demo.