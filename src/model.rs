use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub success: bool,
    pub data: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub success: bool,
    pub data: Vec<TradeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeInformation {
    pub market: String,
    pub price: f64,
    pub size: f64,
    pub side: String,
    pub time: i64,
    pub order_id: u128,
    pub fee_cost: f64,
    pub market_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub success: bool,
    pub data: Vec<VolumeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInformation {
    pub volume_usd: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub success: bool,
    pub data: OrderBookInformation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookInformation {
    pub market: String,
    pub bids: Vec<PriceSize>,
    pub asks: Vec<PriceSize>,
    pub market_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceSize {
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub success: bool,
    pub data: Vec<CandleInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleInformation {
    pub close: f64,
    pub open: f64,
    pub low: f64,
    pub high: f64,
    pub start_time: i64,
    pub market: String,
    pub volume_base: f64,
    pub volume_quote: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub success: bool,
    pub data: Vec<PoolInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolInformation {
    pub name: String,
    #[serde(rename = "pool_identifier")]
    pub pool_identifier: String,
    #[serde(rename = "liquidity_locked")]
    pub liquidity_locked: f64,
    pub apy: f64,
    pub volume: f64,
    pub mints: Vec<String>,

    #[serde(rename = "liquidityA")]
    pub liquidity_a: f64,

    #[serde(rename = "liquidityAinUsd")]
    pub liquidity_a_in_usd: f64,

    #[serde(rename = "liquidityB")]
    pub liquidity_b: f64,

    #[serde(rename = "liquidityBinUsd")]
    pub liquidity_b_in_usd: f64,
    pub supply: f64,
    pub fees: f64,
    pub time: i64,

    pub volume_24h_a: f64,

    pub volume_24h_b: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolTrade {
    pub success: bool,
    pub data: Vec<PoolTradeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolTradeInformation {
    pub signature: String,
    pub symbol_source: String,
    pub pool_source_mint: String,
    pub symbol_destination: String,
    pub pool_destination_mint: String,
    pub amount_in: f64,
    pub amount_out: f64,
    pub pool_mint_authority: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolVolume {
    pub success: bool,
    pub data: Vec<PoolVolumeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolVolumeInformation {
    pub mint_a: String,
    pub mint_b: String,
    pub volume: f64,
    pub time: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolLiquidity {
    pub success: bool,
    pub data: Vec<PoolLiquidityInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolLiquidityInformation {
    pub mint_a: String,
    pub mint_b: String,
    pub liquidity_a: f64,
    pub liquidity_b: f64,
    pub time: i64,

    #[serde(rename = "liquidityAinUsd")]
    pub liquidity_a_in_usd: f64,
    #[serde(rename = "liquidityBinUsd")]
    pub liquidity_b_in_usd: f64,
}
