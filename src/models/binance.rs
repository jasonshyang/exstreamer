use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceMessage {
    SubscriptionAck { result: Option<bool>, id: u64 },
    Trade(BinanceTrade),
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
