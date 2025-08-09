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
    SubscriptionAck(CoinbaseAck),
    Ticker(Box<CoinbaseTicker>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinbaseAck {
    #[serde(rename = "type")]
    pub kind: String, // Should be "subscriptions"
    pub channels: Vec<CoinbaseChannel>,
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
    pub fn create_trade_request(is_sub: bool, channels: Vec<CoinbaseChannel>) -> Self {
        let kind = if is_sub {
            RequestKind::Subscribe
        } else {
            RequestKind::Unsubscribe
        };
        CoinbaseRequest {
            kind,
            params: channels,
            product_ids: vec![], // Not needed as product_ids are specified in channels
        }
    }

    pub fn create_trade_param(symbol: impl Into<String>) -> CoinbaseChannel {
        CoinbaseChannel {
            name: "ticker".to_string(),
            product_ids: vec![symbol.into().to_uppercase()],
        }
    }
}
