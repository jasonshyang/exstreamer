use crate::{
    error::ExStreamError,
    models::{BinanceMessage, BinanceRequest, RequestKind},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone, Default)]
pub struct BinanceBuilder {
    id: Option<u64>,
    params: Vec<String>,
}

impl BinanceBuilder {
    pub const ENDPOINT: &str = "wss://stream.binance.com:9443/ws";

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a subscription to trade data for a specific symbol
    pub fn trade(mut self, symbol: impl Into<String>) -> Self {
        self.params.push(BinanceRequest::create_trade_param(symbol));
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BinanceMessage> {
        if self.params.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        let request = BinanceRequest {
            kind: RequestKind::Subscribe,
            params: self.params,
            id: self.id,
        };

        connect_ws(Self::ENDPOINT, request).await
    }
}
