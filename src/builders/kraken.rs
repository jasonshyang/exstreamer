use crate::{
    error::ExStreamError,
    models::{KrakenChannel, KrakenMessage, KrakenRequest},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone)]
pub struct KrakenBuilder {
    request: KrakenRequest,
}

impl KrakenBuilder {
    pub const ENDPOINT: &str = "wss://ws.kraken.com/v2";

    pub fn new(channel: KrakenChannel) -> Self {
        KrakenBuilder {
            request: KrakenRequest::new_subscribe(channel),
        }
    }

    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.request.add_symbol(symbol);
        self
    }

    pub fn with_symbols(mut self, symbols: Vec<impl Into<String>>) -> Self {
        self.request.add_symbols(symbols);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<KrakenMessage> {
        if self.request.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        connect_ws(Self::ENDPOINT, self.request).await
    }
}
