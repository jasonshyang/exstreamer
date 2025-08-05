const DEFAULT_ORDERBOOK_DEPTH: u64 = 50;
const BINANCE_ENDPOINT: &str = "wss://stream.binance.com:9443/ws";
const BYBIT_ENDPOINT: &str = "wss://stream.bybit.com/v5/public/spot";

#[derive(Debug, Clone)]
pub struct Subscription {
    pub kind: SubscriptionKind,
    pub symbol: String,
    pub id: Option<u64>,
    pub depth: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum SubscriptionSource {
    Binance,
    Bybit,
}

#[derive(Debug, Clone, Copy)]
pub enum SubscriptionKind {
    Trade,
    OrderBook,
}

impl SubscriptionSource {
    pub fn endpoint(&self) -> &str {
        match self {
            SubscriptionSource::Binance => BINANCE_ENDPOINT,
            SubscriptionSource::Bybit => BYBIT_ENDPOINT,
        }
    }
}

impl Subscription {
    pub fn new(
        kind: SubscriptionKind,
        symbol: impl Into<String>,
        id: Option<u64>,
        depth: Option<u64>,
    ) -> Self {
        Self {
            kind,
            symbol: symbol.into(),
            id,
            depth,
        }
    }

    pub fn to_subscription_msg(&self, source: &SubscriptionSource) -> String {
        let topic = self.to_topic(source);
        match source {
            SubscriptionSource::Binance => match self.id {
                Some(id) => serde_json::json!({
                    "method": "SUBSCRIBE",
                    "params": [topic],
                    "id": id
                })
                .to_string(),
                None => serde_json::json!({
                    "method": "SUBSCRIBE",
                    "params": [topic],
                    "id": null
                })
                .to_string(),
            },
            SubscriptionSource::Bybit => match self.id {
                Some(id) => serde_json::json!({
                    "op": "subscribe",
                    "args": [topic],
                    "req_id": id.to_string()
                })
                .to_string(),
                None => serde_json::json!({
                    "op": "subscribe",
                    "args": [topic],
                })
                .to_string(),
            },
        }
    }

    pub fn to_unsubscription_msg(&self, source: &SubscriptionSource) -> String {
        let topic = self.to_topic(source);
        match source {
            SubscriptionSource::Binance => match self.id {
                Some(id) => serde_json::json!({
                    "method": "UNSUBSCRIBE",
                    "params": [topic],
                    "id": id
                })
                .to_string(),
                None => serde_json::json!({
                    "method": "UNSUBSCRIBE",
                    "params": [topic],
                    "id": null
                })
                .to_string(),
            },
            SubscriptionSource::Bybit => match self.id {
                Some(id) => serde_json::json!({
                    "op": "unsubscribe",
                    "args": [topic],
                    "req_id": id.to_string()
                })
                .to_string(),
                None => serde_json::json!({
                    "op": "unsubscribe",
                    "args": [topic],
                })
                .to_string(),
            },
        }
    }

    pub fn to_topic(&self, source: &SubscriptionSource) -> String {
        match source {
            SubscriptionSource::Binance => self.binance_topic(),
            SubscriptionSource::Bybit => self.bybit_topic(),
        }
    }

    fn binance_topic(&self) -> String {
        match self.kind {
            SubscriptionKind::Trade => format!("{}@trade", self.symbol.to_lowercase()),
            SubscriptionKind::OrderBook => {
                panic!("OrderBook subscription not implemented for Binance")
            }
        }
    }

    fn bybit_topic(&self) -> String {
        match self.kind {
            SubscriptionKind::Trade => format!("publicTrade.{}", self.symbol.to_uppercase()),
            SubscriptionKind::OrderBook => {
                format!(
                    "orderbook.{}.{}",
                    self.depth.unwrap_or(DEFAULT_ORDERBOOK_DEPTH),
                    self.symbol.to_uppercase()
                )
            }
        }
    }
}
