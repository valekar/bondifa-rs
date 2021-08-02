use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    success: bool,
    data: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    success: bool,
    data: Vec<TradeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeInformation {
    market: String,
    price: f64,
    size: f64,
    side: String,
    time: i64,
    order_id: u128,
    fee_cost: f64,
    market_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    success: bool,
    data: Vec<VolumeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInformation {
    volume_usd: f64,
    volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    success: bool,
    data: OrderBookInformation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookInformation {
    market: String,
    bids: Vec<PriceSize>,
    asks: Vec<PriceSize>,
    market_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceSize {
    price: f64,
    size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    success: bool,
    data: Vec<CandleInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleInformation {
    close: f64,
    open: f64,
    low: f64,
    high: f64,
    start_time: i64,
    market: String,
    volume_base: f64,
    volume_quote: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    success: bool,
    data: Vec<PoolInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolInformation {
    name: String,
    #[serde(rename = "pool_identifier")]
    pool_identifier: String,
    #[serde(rename = "liquidity_locked")]
    liquidity_locked: f64,
    apy: f64,
    volume: f64,
    mints: Vec<String>,

    #[serde(rename = "liquidityA")]
    liquidity_a: f64,

    #[serde(rename = "liquidityAinUsd")]
    liquidity_a_in_usd: f64,

    #[serde(rename = "liquidityB")]
    liquidity_b: f64,

    #[serde(rename = "liquidityBinUsd")]
    liquidity_b_in_usd: f64,
    supply: f64,
    fees: f64,
    time: i64,

    volume_24h_a: f64,

    volume_24h_b: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolTrade {
    success: bool,
    data: Vec<PoolTradeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolTradeInformation {
    signature: String,
    symbol_source: String,
    pool_source_mint: String,
    symbol_destination: String,
    pool_destination_mint: String,
    amount_in: f64,
    amount_out: f64,
    pool_mint_authority: String,
    time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolVolume {
    success: bool,
    data: Vec<PoolVolumeInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolVolumeInformation {
    mint_a: String,
    mint_b: String,
    volume: f64,
    time: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolLiquidity {
    success: bool,
    data: Vec<PoolLiquidityInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolLiquidityInformation {
    mint_a: String,
    mint_b: String,
    liquidity_a: f64,
    liquidity_b: f64,
    time: i64,

    #[serde(rename = "liquidityAinUsd")]
    liquidity_a_in_usd: f64,
    #[serde(rename = "liquidityBinUsd")]
    liquidity_b_in_usd: f64,
}
