use serde::{Deserialize, Serialize};

use crate::models::{RequestKind, to_lower};

#[derive(Serialize, Debug, Clone)]
pub struct KrakenRequest {
    #[serde(rename = "method", with = "to_lower")]
    pub kind: RequestKind,
    pub params: KrakenParams,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KrakenParams {
    Trade(KrakenTradeParams),
    L3(KrakenL3Params),
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
pub struct KrakenL3Params {
    /// This must be "level3" for L3 requests
    channel: KrakenChannel,
    pub symbol: Vec<String>,
    /// Possible values: [10, 100, 1000]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    /// Request a snapshot after subscribing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    pub token: String, // Authentication token
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum KrakenChannel {
    Trade,
    #[serde(rename = "level3")]
    L3,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KrakenMessage {
    SubscriptionAck {
        #[serde(rename = "method", with = "to_lower")]
        kind: RequestKind,
        result: KrakenParams,
        success: bool,
        error: String,
        time_in: String,
        time_out: String,
        req_id: Option<u64>,
    },
    Event(KrakenEvent),
    Heartbeat {
        channel: String,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KrakenEvent {
    pub channel: KrakenChannel,
    #[serde(rename = "type")]
    pub kind: KrakenEventKind,
    pub data: Vec<KrakenData>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum KrakenEventKind {
    Snapshot,
    Update,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KrakenData {
    Trade(KrakenTradeData),
    Book(KrakenBook),
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

#[derive(Deserialize, Debug, Clone)]
pub struct KrakenBook {
    pub symbol: String,
    pub checksum: u64,
    pub bids: Vec<KrakenOrderEntry>,
    pub asks: Vec<KrakenOrderEntry>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KrakenOrderEntry {
    pub order_id: String,
    #[serde(rename = "limit_price")]
    pub price: f64,
    #[serde(rename = "order_qty")]
    pub size: f64,
    pub timestamp: String, // Format: RFC3339
}

impl KrakenRequest {
    pub fn new(kind: RequestKind, params: KrakenParams) -> Self {
        KrakenRequest {
            kind,
            params,
            id: None,
        }
    }

    pub fn new_subscribe(channel: KrakenChannel) -> Self {
        let params = match channel {
            KrakenChannel::Trade => KrakenParams::Trade(KrakenTradeParams {
                channel,
                symbol: Vec::new(),
                snapshot: None,
            }),
            KrakenChannel::L3 => KrakenParams::L3(KrakenL3Params {
                channel,
                symbol: Vec::new(),
                depth: None,
                snapshot: Some(true),
                token: String::new(), // Token should be set later
            }),
        };

        KrakenRequest {
            kind: RequestKind::Subscribe,
            params,
            id: None,
        }
    }

    pub fn new_unsubscribe(channel: KrakenChannel) -> Self {
        let params = match channel {
            KrakenChannel::Trade => KrakenParams::Trade(KrakenTradeParams {
                channel,
                symbol: Vec::new(),
                snapshot: None,
            }),
            KrakenChannel::L3 => KrakenParams::L3(KrakenL3Params {
                channel,
                symbol: Vec::new(),
                depth: None,
                snapshot: Some(true),
                token: String::new(), // Token should be set later
            }),
        };

        KrakenRequest {
            kind: RequestKind::Unsubscribe,
            params,
            id: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.params {
            KrakenParams::Trade(ref params) => params.symbol.is_empty(),
            KrakenParams::L3(ref params) => params.symbol.is_empty(),
        }
    }

    pub fn is_missing_auth(&self) -> bool {
        if let KrakenParams::L3(ref params) = self.params {
            params.token.is_empty()
        } else {
            false
        }
    }

    pub fn is_auth_required(&self) -> bool {
        matches!(self.params, KrakenParams::L3(_))
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn set_token(&mut self, token: String) {
        if let KrakenParams::L3(ref mut params) = self.params {
            params.token = token;
        }
    }

    pub fn set_depth(&mut self, depth: u64) {
        if let KrakenParams::L3(ref mut params) = self.params {
            params.depth = Some(depth);
        }
    }

    pub fn add_symbol(&mut self, symbol: impl Into<String>) {
        match &mut self.params {
            KrakenParams::Trade(params) => params.symbol.push(symbol.into().to_uppercase()),
            KrakenParams::L3(params) => params.symbol.push(symbol.into().to_uppercase()),
        }
    }

    pub fn add_symbols(&mut self, symbols: Vec<impl Into<String>>) {
        match &mut self.params {
            KrakenParams::Trade(params) => {
                for symbol in symbols {
                    params.symbol.push(symbol.into().to_uppercase());
                }
            }
            KrakenParams::L3(params) => {
                for symbol in symbols {
                    params.symbol.push(symbol.into().to_uppercase());
                }
            }
        }
    }
}
