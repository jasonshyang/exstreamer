use serde::{Deserialize, Serialize};

use crate::models::{RequestKind, to_upper};

#[derive(Serialize, Debug, Clone)]
pub struct BinanceRequest {
    #[serde(rename = "method", with = "to_upper")]
    pub kind: RequestKind,
    pub params: Vec<String>,
    pub id: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceMessage {
    SubscriptionAck(BinanceAck),
    Trade(BinanceTrade),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BinanceAck {
    pub result: Option<bool>,
    pub id: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceTrade {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,
    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Symbol name, e.g. BNBBTC
    #[serde(rename = "s")]
    pub symbol: String,
    /// Trade ID
    #[serde(rename = "t")]
    pub trade_id: u64,
    /// Price of the trade
    #[serde(rename = "p")]
    pub price: String,
    /// Quantity of the trade
    #[serde(rename = "q")]
    pub quantity: String,
    /// Trade time
    #[serde(rename = "T")]
    pub trade_time: u64,
    /// Is the buyer the market maker?
    #[serde(rename = "m")]
    pub is_market_maker: bool,
    /// Ignore field
    #[serde(rename = "M")]
    pub ignore: bool,
}

impl BinanceRequest {
    pub fn create_trade_request(is_sub: bool, symbol: impl Into<String>, id: Option<u64>) -> Self {
        let params = vec![Self::create_trade_param(symbol)];
        let kind = if is_sub {
            RequestKind::Subscribe
        } else {
            RequestKind::Unsubscribe
        };
        BinanceRequest { kind, params, id }
    }

    pub fn create_trade_param(symbol: impl Into<String>) -> String {
        format!("{}@trade", symbol.into().to_lowercase())
    }
}
