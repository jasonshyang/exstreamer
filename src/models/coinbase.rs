use serde::{Deserialize, Serialize};

use crate::models::{RequestKind, to_lower};

#[derive(Serialize, Debug, Clone)]
pub struct CoinbaseRequest {
    #[serde(rename = "type", with = "to_lower")]
    pub kind: RequestKind,
    #[serde(rename = "channels")]
    pub params: Vec<CoinbaseChannel>,
    // This is not really needed if we specify product_ids in channels
    pub product_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CoinbaseMessage {
    SubscriptionAck {
        #[serde(rename = "type")]
        kind: String, // Should be "subscriptions"
        channels: Vec<CoinbaseChannel>,
    },
    Ticker(Box<CoinbaseTicker>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinbaseChannel {
    pub name: String,
    pub product_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinbaseTicker {
    #[serde(rename = "type")]
    pub kind: String,
    pub sequence: u64,
    pub product_id: String,
    pub price: String,
    pub open_24h: String,
    pub volume_24h: String,
    pub low_24h: String,
    pub high_24h: String,
    pub volume_30d: String,
    pub best_bid: String,
    pub best_bid_size: String,
    pub best_ask: String,
    pub best_ask_size: String,
    pub side: String,
    pub time: String,
    pub trade_id: u64,
    pub last_size: String,
}

impl CoinbaseRequest {
    pub fn trade_request(kind: RequestKind, symbol: impl Into<String>) -> Self {
        let channels = vec![Self::trade_param(symbol)];
        CoinbaseRequest {
            kind,
            params: channels,
            product_ids: Vec::new(), // Not needed as product_ids are specified in channels
        }
    }

    pub fn trade_param(symbol: impl Into<String>) -> CoinbaseChannel {
        CoinbaseChannel {
            name: "ticker".to_string(),
            product_ids: vec![symbol.into().to_uppercase()],
        }
    }

    pub fn new(kind: RequestKind, params: Vec<CoinbaseChannel>) -> Self {
        Self {
            kind,
            params,
            product_ids: Vec::new(),
        }
    }

    pub fn new_subscribe() -> Self {
        Self {
            kind: RequestKind::Subscribe,
            params: Vec::new(),
            product_ids: Vec::new(),
        }
    }

    pub fn new_unsubscribe() -> Self {
        Self {
            kind: RequestKind::Unsubscribe,
            params: Vec::new(),
            product_ids: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn with_trade(mut self, symbol: impl Into<String>) -> Self {
        self.add_trade(symbol);
        self
    }

    pub fn with_trades(mut self, symbols: Vec<impl Into<String>>) -> Self {
        self.add_trades(symbols);
        self
    }

    pub fn add_trade(&mut self, symbol: impl Into<String>) {
        let channel = CoinbaseChannel {
            name: "ticker".to_string(),
            product_ids: vec![symbol.into().to_uppercase()],
        };

        self.params.push(channel);
    }

    pub fn add_trades(&mut self, symbols: Vec<impl Into<String>>) {
        for symbol in symbols {
            self.add_trade(symbol);
        }
    }
}
