mod binance;
mod bybit;
mod coinbase;
mod kraken;

pub use binance::*;
pub use bybit::*;
pub use coinbase::*;
pub use kraken::*;

use crate::models::KrakenChannel;

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

    pub fn kraken(channel: KrakenChannel) -> KrakenBuilder {
        KrakenBuilder::new(channel)
    }
}
