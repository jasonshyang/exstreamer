use crate::{
    error::ExStreamError,
    models::{CoinbaseChannel, CoinbaseMessage, CoinbaseRequest, RequestKind},
    transport::{ConnectionResult, connect_ws},
};

#[derive(Debug, Clone, Default)]
pub struct CoinbaseBuilder {
    params: Vec<CoinbaseChannel>,
}

impl CoinbaseBuilder {
    pub const ENDPOINT: &str = "wss://ws-feed.exchange.coinbase.com";

    /// Add a subscription to trade data for a specific symbol
    pub fn trade(mut self, symbol: impl Into<String>) -> Self {
        let channel = CoinbaseChannel {
            name: "ticker".to_string(),
            product_ids: vec![symbol.into().to_uppercase()],
        };
        self.params.push(channel);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<CoinbaseMessage> {
        if self.params.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        let request = CoinbaseRequest {
            kind: RequestKind::Subscribe,
            params: self.params,
            product_ids: vec![], // Not needed as product_ids are specified in channels
        };

        connect_ws(Self::ENDPOINT, request).await
    }
}
