use crate::{
    error::ExStreamError,
    models::{CoinbaseMessage, CoinbaseRequest},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone)]
pub struct CoinbaseBuilder {
    request: CoinbaseRequest,
}

impl CoinbaseBuilder {
    pub const ENDPOINT: &str = "wss://ws-feed.exchange.coinbase.com";

    pub fn with_trade(mut self, symbol: impl Into<String>) -> Self {
        self.request.add_trade(symbol);
        self
    }

    pub fn with_trades(mut self, symbols: Vec<impl Into<String>>) -> Self {
        self.request.add_trades(symbols);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<CoinbaseMessage> {
        if self.request.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        connect_ws(Self::ENDPOINT, self.request).await
    }
}

impl Default for CoinbaseBuilder {
    fn default() -> Self {
        CoinbaseBuilder {
            request: CoinbaseRequest::new_subscribe(),
        }
    }
}
