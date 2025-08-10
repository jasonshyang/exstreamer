use serde::{Deserialize, Serialize};

use crate::models::{RequestKind, to_lower};

pub type BybitOrderEntry = Vec<String>; // [price, size]

#[derive(Serialize, Debug, Clone)]
pub struct BybitRequest {
    #[serde(rename = "op", with = "to_lower")]
    pub kind: RequestKind,
    #[serde(rename = "args")]
    pub params: Vec<String>,
    #[serde(rename = "req_id")]
    pub id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BybitMessage {
    SubscriptionAck {
        success: bool,
        #[serde(rename = "ret_msg")]
        message: String,
        #[serde(rename = "conn_id")]
        connection_id: String,
        #[serde(rename = "req_id")]
        request_id: Option<String>,
        #[serde(rename = "op")]
        operation: String,
    },
    OrderBook(BybitOrderBook),
    Trade(BybitTrade),
}

#[derive(Deserialize, Debug)]
pub struct BybitOrderBook {
    /// Topic name
    pub topic: String,
    /// The timestamp (ms) that the system generates the data
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// Data type: snapshot,delta
    #[serde(rename = "type")]
    pub data_type: BybitDataType,
    /// Order book data
    #[serde(rename = "data")]
    pub data: BybitOrderBookData,
    /// The timestamp from the matching engine when this orderbook data is produced.
    /// It can be correlated with T from public trade channel
    #[serde(rename = "cts")]
    pub correlated_timestamp: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BybitDataType {
    Snapshot,
    Delta,
}

#[derive(Deserialize, Debug)]
pub struct BybitOrderBookData {
    /// Symbol name, e.g. SOLUSDT_SOL/USDT
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bids. For snapshot stream. Sorted by price in descending order
    /// b[0] is the price, b[1] is the size
    /// The delta data has size=0, which means that all quotations for this price have been filled or cancelled
    #[serde(rename = "b")]
    pub bids: Vec<BybitOrderEntry>,
    /// Asks. For snapshot stream. Sorted by price in ascending order
    #[serde(rename = "a")]
    pub asks: Vec<BybitOrderEntry>,
    /// Update ID, if receive "u"=1, that is a snapshot data due to the restart of the service.
    /// Local orderbook should be reset
    #[serde(rename = "u")]
    pub update_id: u64,
    /// Cross sequence
    #[serde(rename = "seq")]
    pub sequence: u64,
}

#[derive(Deserialize, Debug)]
pub struct BybitTrade {
    /// Topic name
    pub topic: String,
    /// The timestamp (ms) that the system generates the data
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// Data type: trade
    #[serde(rename = "type")]
    pub data_type: BybitDataType,
    /// Trade data
    #[serde(rename = "data")]
    pub data: Vec<BybitTradeData>,
}

#[derive(Deserialize, Debug)]
pub struct BybitTradeData {
    /// The timestamp (ms) that the order is filled
    #[serde(rename = "T")]
    pub timestamp: u64,
    /// Symbol name, e.g. SOLUSDT_SOL/USDT
    #[serde(rename = "s")]
    pub symbol: String,
    /// Side of taker
    #[serde(rename = "S")]
    pub side: String,
    /// Trade ID
    #[serde(rename = "v")]
    pub size: String,
    /// Price
    #[serde(rename = "p")]
    pub price: String,
    /// Direction of price change, this is documented but not provided
    // #[serde(rename = "L")]
    // pub direction: String,
    /// Trade ID
    #[serde(rename = "i")]
    pub trade_id: String,
    /// Undocumented on Bybit documentation
    #[serde(rename = "BT")]
    pub bt: bool,
    /// Undocumented on Bybit documentation
    #[serde(rename = "RPI")]
    pub rpi: bool,
}

impl BybitRequest {
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

    pub fn with_id(mut self, id_str: String) -> Self {
        self.id = Some(id_str);
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

    pub fn with_orderbook(mut self, symbol: impl Into<String>, depth: u64) -> Self {
        self.add_orderbook(symbol, depth);
        self
    }

    pub fn with_orderbooks(mut self, symbols: Vec<impl Into<String>>, depth: u64) -> Self {
        self.add_orderbooks(symbols, depth);
        self
    }

    pub fn add_trade(&mut self, symbol: impl Into<String>) {
        self.params.push(Self::format_trade(symbol));
    }

    pub fn add_trades(&mut self, symbols: Vec<impl Into<String>>) {
        for symbol in symbols {
            self.add_trade(symbol);
        }
    }

    pub fn add_orderbook(&mut self, symbol: impl Into<String>, depth: u64) {
        self.params.push(Self::format_orderbook(symbol, depth));
    }

    pub fn add_orderbooks(&mut self, symbols: Vec<impl Into<String>>, depth: u64) {
        for symbol in symbols {
            self.add_orderbook(symbol, depth);
        }
    }

    fn format_trade(symbol: impl Into<String>) -> String {
        format!("publicTrade.{}", symbol.into().to_uppercase())
    }

    fn format_orderbook(symbol: impl Into<String>, depth: u64) -> String {
        format!("orderbook.{}.{}", depth, symbol.into().to_uppercase())
    }
}
