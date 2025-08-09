use crate::{
    error::ExStreamError,
    models::{BybitMessage, BybitRequest, RequestKind},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone, Default)]
pub struct BybitBuilder {
    id: Option<u64>,
    params: Vec<String>,
}

impl BybitBuilder {
    pub const ENDPOINT: &str = "wss://stream.bybit.com/v5/public/spot";

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a subscription to trade data for a specific symbol
    pub fn trade(mut self, symbol: impl Into<String>) -> Self {
        self.params.push(BybitRequest::create_trade_param(symbol));
        self
    }

    /// Add a subscription to order book data for a specific symbol and depth
    pub fn orderbook(mut self, symbol: impl Into<String>, depth: u64) -> Self {
        self.params
            .push(BybitRequest::create_orderbook_param(symbol, depth));
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BybitMessage> {
        if self.params.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        let request = BybitRequest {
            kind: RequestKind::Subscribe,
            params: self.params,
            id: self.id.map(|id| id.to_string()),
        };

        connect_ws(Self::ENDPOINT, request).await
    }
}
