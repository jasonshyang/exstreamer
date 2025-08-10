use crate::{
    error::ExStreamError,
    models::{BybitMessage, BybitRequest},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone)]
pub struct BybitBuilder {
    request: BybitRequest,
}

impl BybitBuilder {
    pub const ENDPOINT: &str = "wss://stream.bybit.com/v5/public/spot";

    pub fn with_id(mut self, id_str: String) -> Self {
        self.request.id = Some(id_str);
        self
    }

    pub fn with_trade(mut self, symbol: impl Into<String>) -> Self {
        self.request.add_trade(symbol);
        self
    }

    pub fn with_trades(mut self, symbols: Vec<impl Into<String>>) -> Self {
        self.request.add_trades(symbols);
        self
    }

    pub fn with_orderbook(mut self, symbol: impl Into<String>, depth: u64) -> Self {
        self.request.add_orderbook(symbol, depth);
        self
    }

    pub fn with_orderbooks(mut self, symbols: Vec<impl Into<String>>, depth: u64) -> Self {
        self.request.add_orderbooks(symbols, depth);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BybitMessage> {
        if self.request.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        connect_ws(Self::ENDPOINT, self.request).await
    }
}

impl Default for BybitBuilder {
    fn default() -> Self {
        BybitBuilder {
            request: BybitRequest::new_subscribe(),
        }
    }
}
