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
    pub const ENDPOINT_AUTH: &str = "wss://ws-auth.kraken.com/v2";

    pub fn new(channel: KrakenChannel) -> Self {
        KrakenBuilder {
            request: KrakenRequest::new_subscribe(channel),
        }
    }

    /// Only used for channels requiring authentication
    pub fn with_token(mut self, token: String) -> Self {
        self.request.set_token(token);
        self
    }

    /// Set the depth for L3 channels, possible values 10, 100, 1000
    pub fn with_depth(mut self, depth: u64) -> Self {
        self.request.set_depth(depth);
        self
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.request.set_id(id);
        self
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

        if self.request.is_missing_auth() {
            return Err(ExStreamError::MissingAuth);
        }

        let endpoint = match self.request.is_auth_required() {
            true => Self::ENDPOINT_AUTH,
            false => Self::ENDPOINT,
        };

        connect_ws(endpoint, self.request).await
    }
}
