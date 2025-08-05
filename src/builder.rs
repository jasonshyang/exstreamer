use crate::{
    error::ExStreamError,
    models::{BinanceMessage, BybitMessage, Subscription, SubscriptionKind, SubscriptionSource},
    transport::{ConnectionResult, connect_ws},
};

/// Builder for creating exchange streams.
pub struct StreamBuilder;

pub struct BinanceBuilder {
    id: Option<u64>,
    subscriptions: Vec<Subscription>,
}

pub struct BybitBuilder {
    id: Option<u64>,
    depth: Option<u64>,
    subscriptions: Vec<Subscription>,
}

impl StreamBuilder {
    /// Start building a Binance stream
    pub fn binance() -> BinanceBuilder {
        BinanceBuilder {
            id: None,
            subscriptions: Vec::new(),
        }
    }

    /// Start building a Bybit stream
    pub fn bybit() -> BybitBuilder {
        BybitBuilder {
            id: None,
            depth: None,
            subscriptions: Vec::new(),
        }
    }
}

impl BinanceBuilder {
    const SOURCE: SubscriptionSource = SubscriptionSource::Binance;

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a subscription to trade data for a specific symbol
    pub fn trade(mut self, symbol: impl Into<String>) -> Self {
        let subscription = Subscription {
            kind: SubscriptionKind::Trade,
            symbol: symbol.into(),
            id: self.id,
            depth: None,
        };
        self.subscriptions.push(subscription);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BinanceMessage> {
        if self.subscriptions.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        let subscription_msg = self.build_subscription_msg();

        connect_ws(Self::SOURCE, subscription_msg).await
    }

    fn build_subscription_msg(&self) -> String {
        let topics: Vec<String> = self
            .subscriptions
            .iter()
            .map(|sub| sub.to_topic(&Self::SOURCE))
            .collect();

        match self.id {
            Some(id) => serde_json::json!({
                "method": "SUBSCRIBE",
                "params": topics,
                "id": id
            })
            .to_string(),
            None => serde_json::json!({
                "method": "SUBSCRIBE",
                "params": topics,
                "id": null
            })
            .to_string(),
        }
    }
}

impl BybitBuilder {
    const SOURCE: SubscriptionSource = SubscriptionSource::Bybit;

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_depth(mut self, depth: u64) -> Self {
        self.depth = Some(depth);
        self
    }

    /// Add a subscription to trade data for a specific symbol
    pub fn trade(mut self, symbol: impl Into<String>) -> Self {
        let subscription = Subscription {
            kind: SubscriptionKind::Trade,
            symbol: symbol.into(),
            id: self.id,
            depth: None,
        };
        self.subscriptions.push(subscription);
        self
    }

    /// Add a subscription to order book data for a specific symbol
    pub fn orderbook(mut self, symbol: impl Into<String>) -> Self {
        let subscription = Subscription {
            kind: SubscriptionKind::OrderBook,
            symbol: symbol.into(),
            id: self.id,
            depth: self.depth,
        };
        self.subscriptions.push(subscription);
        self
    }

    // Connect and return the stream
    pub async fn connect(self) -> ConnectionResult<BybitMessage> {
        if self.subscriptions.is_empty() {
            return Err(ExStreamError::EmptySubscriptionList);
        }

        let subscription_msg = self.build_subscription_msg();
        connect_ws(Self::SOURCE, subscription_msg).await
    }

    fn build_subscription_msg(&self) -> String {
        let topics: Vec<String> = self
            .subscriptions
            .iter()
            .map(|sub| sub.to_topic(&Self::SOURCE))
            .collect();

        match self.id {
            Some(id) => serde_json::json!({
                "op": "subscribe",
                "args": topics,
                "req_id": id.to_string()
            })
            .to_string(),
            None => serde_json::json!({
                "op": "subscribe",
                "args": topics
            })
            .to_string(),
        }
    }
}
