use crate::{
    error::ExStreamError,
    models::{BinanceMessage, BinanceRequest},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone)]
pub struct BinanceBuilder {
    request: BinanceRequest,
}

impl BinanceBuilder {
    pub const ENDPOINT: &str = "wss://stream.binance.com:9443/ws";

    pub fn new() -> Self {
        BinanceBuilder {
            request: BinanceRequest::new_subscribe(),
        }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.request.id = Some(id);
        self
    }

    pub fn with_trade(mut self, symbol: impl Into<String>) -> Self {
        self.request.add_trade(symbol);
        self
    }

    pub fn with_trades(mut self, symbols: Vec<String>) -> Self {
        self.request.add_trades(symbols);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BinanceMessage> {
        if self.request.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        connect_ws(Self::ENDPOINT, self.request).await
    }
}

impl Default for BinanceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
