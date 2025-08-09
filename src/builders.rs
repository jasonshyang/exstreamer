mod binance;
mod bybit;
mod coinbase;

pub use binance::*;
pub use bybit::*;
pub use coinbase::*;

/// Builder for creating exchange streams.
pub struct StreamBuilder;

impl StreamBuilder {
    /// Start building a Binance stream
    pub fn binance() -> BinanceBuilder {
        BinanceBuilder::default()
    }

    /// Start building a Bybit stream
    pub fn bybit() -> BybitBuilder {
        BybitBuilder::default()
    }

    /// Start building a Coinbase stream
    pub fn coinbase() -> CoinbaseBuilder {
        CoinbaseBuilder::default()
    }
}
