use serde::{Deserialize, Serialize};

use crate::models::{RequestKind, to_lower};

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KrakenRequest {
    Trade(KrakenTradeRequest),
}

#[derive(Serialize, Debug, Clone)]
pub struct KrakenTradeRequest {
    #[serde(rename = "method", with = "to_lower")]
    pub kind: RequestKind,
    pub params: KrakenTradeParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrakenTradeParams {
    /// This must be "trade" for trade requests
    channel: KrakenChannel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum KrakenChannel {
    Trade,
    #[serde(rename = "book")]
    OrderbookL2,
    #[serde(rename = "level3")]
    OrderbookL3,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KrakenMessage {
    SubscriptionAck {
        #[serde(rename = "method", with = "to_lower")]
        kind: RequestKind,
        result: KrakenTradeParams,
        success: bool,
        error: String,
        time_in: String,
        time_out: String,
        req_id: Option<u64>,
    },
    Trade(KrakenTrade),
    Heartbeat {
        channel: String,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KrakenTrade {
    pub channel: KrakenChannel,
    #[serde(rename = "type")]
    pub kind: String, // "snapshot" or "update"
    pub data: Vec<KrakenTradeData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KrakenTradeData {
    pub symbol: String,
    pub side: String,
    #[serde(rename = "qty")]
    pub size: f64,
    pub price: f64,
    #[serde(rename = "ord_type")]
    pub order_type: String,
    pub trade_id: u64,
    pub timestamp: String, // Format: RFC3339
}

impl KrakenRequest {
    pub fn new(channel: KrakenChannel, kind: RequestKind, params: KrakenTradeParams) -> Self {
        match channel {
            KrakenChannel::Trade => KrakenRequest::Trade(KrakenTradeRequest {
                kind,
                params,
                id: None,
            }),
            KrakenChannel::OrderbookL2 => unimplemented!("Orderbook L2 not implemented yet"),
            KrakenChannel::OrderbookL3 => unimplemented!("Orderbook L3 not implemented yet"),
        }
    }

    pub fn new_subscribe(channel: KrakenChannel) -> Self {
        match channel {
            KrakenChannel::Trade => KrakenRequest::Trade(KrakenTradeRequest::new_subscribe()),
            KrakenChannel::OrderbookL2 => unimplemented!("Orderbook L2 not implemented yet"),
            KrakenChannel::OrderbookL3 => unimplemented!("Orderbook L3 not implemented yet"),
        }
    }

    pub fn new_unsubscribe(channel: KrakenChannel) -> Self {
        match channel {
            KrakenChannel::Trade => KrakenRequest::Trade(KrakenTradeRequest::new_unsubscribe()),
            KrakenChannel::OrderbookL2 => unimplemented!("Orderbook L2 not implemented yet"),
            KrakenChannel::OrderbookL3 => unimplemented!("Orderbook L3 not implemented yet"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            KrakenRequest::Trade(req) => req.is_empty(),
        }
    }

    pub fn with_id(self, id: u64) -> Self {
        match self {
            KrakenRequest::Trade(req) => req.with_id(id).into(),
        }
    }

    pub fn with_symbol(self, symbol: impl Into<String>) -> Self {
        match self {
            KrakenRequest::Trade(req) => req.with_symbol(symbol).into(),
        }
    }

    pub fn with_symbols(self, symbols: Vec<impl Into<String>>) -> Self {
        match self {
            KrakenRequest::Trade(req) => req.with_symbols(symbols).into(),
        }
    }

    pub fn add_symbol(&mut self, symbol: impl Into<String>) {
        match self {
            KrakenRequest::Trade(req) => req.add_symbol(symbol),
        }
    }

    pub fn add_symbols(&mut self, symbols: Vec<impl Into<String>>) {
        match self {
            KrakenRequest::Trade(req) => req.add_symbols(symbols),
        }
    }
}

impl KrakenTradeRequest {
    pub fn new_subscribe() -> Self {
        let params = KrakenTradeParams {
            channel: KrakenChannel::Trade,
            symbol: Vec::new(),
            snapshot: None,
        };

        Self {
            kind: RequestKind::Subscribe,
            params,
            id: None,
        }
    }

    pub fn new_unsubscribe() -> Self {
        let params = KrakenTradeParams {
            channel: KrakenChannel::Trade,
            symbol: Vec::new(),
            snapshot: None,
        };

        Self {
            kind: RequestKind::Unsubscribe,
            params,
            id: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.params.symbol.is_empty()
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.add_symbol(symbol);
        self
    }

    pub fn with_symbols(mut self, symbols: Vec<impl Into<String>>) -> Self {
        self.add_symbols(symbols);
        self
    }

    pub fn add_symbol(&mut self, symbol: impl Into<String>) {
        self.params.symbol.push(symbol.into().to_uppercase());
    }

    pub fn add_symbols(&mut self, symbols: Vec<impl Into<String>>) {
        for symbol in symbols {
            self.params.symbol.push(symbol.into().to_uppercase());
        }
    }
}

impl From<KrakenTradeRequest> for KrakenRequest {
    fn from(req: KrakenTradeRequest) -> Self {
        KrakenRequest::Trade(req)
    }
}
