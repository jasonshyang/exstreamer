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
    pub fn new(kind: RequestKind, params: Vec<impl Into<String>>) -> Self {
        let params = params.into_iter().map(|p| p.into()).collect();

        Self {
            kind,
            params,
            id: None,
        }
    }

    pub fn new_subscribe() -> Self {
        Self {
            kind: RequestKind::Subscribe,
            params: Vec::new(),
            id: None,
        }
    }

    pub fn new_unsubscribe() -> Self {
        Self {
            kind: RequestKind::Unsubscribe,
            params: Vec::new(),
            id: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
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
        self.params.push(Self::format_trade(symbol));
    }

    pub fn add_trades(&mut self, symbols: Vec<impl Into<String>>) {
        for symbol in symbols {
            self.params.push(Self::format_trade(symbol));
        }
    }

    fn format_trade(symbol: impl Into<String>) -> String {
        format!("{}@trade", symbol.into().to_lowercase())
    }
}
