use serde::Deserialize;

use crate::types::WsSubscription;

pub const BINANCE_ENDPOINT: &str = "wss://stream.binance.com:9443/ws";

pub struct BinanceConfig {
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BinanceMessage {
    SubscriptionAck { result: Option<bool>, id: u64 },
    Trade(BinanceTrade),
}

#[derive(Deserialize, Debug)]
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

pub struct BinanceSubscription {
    id: u64,
    config: BinanceConfig,
}

impl WsSubscription for BinanceSubscription {
    type Message = BinanceMessage;
    type Config = BinanceConfig;

    fn new(config: BinanceConfig, id: u64) -> Self {
        Self { id, config }
    }

    fn name(&self) -> &'static str {
        "binance"
    }

    fn endpoint(&self) -> &'static str {
        BINANCE_ENDPOINT
    }

    fn sub_msg(&self) -> String {
        let topic = format!("{}@trade", self.config.symbol);

        serde_json::json!({
            "method": "SUBSCRIBE",
            "params": [topic],
            "id": self.id
        })
        .to_string()
    }

    fn unsub_msg(&mut self) -> String {
        let topic = format!("{}@trade", self.config.symbol);

        serde_json::json!({
            "method": "UNSUBSCRIBE",
            "params": [topic],
            "id": self.id
        })
        .to_string()
    }
}
