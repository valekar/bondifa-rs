#[allow(clippy::all)]
pub enum API {
    Serum(Serum),
}

pub enum Serum {
    Pairs,
    Trades(String),
    AddressTrade(String),
    AllRecentTrades,
    Volumes(String),
    OrderBooks(String),
    HistoricalPrices(String),
    Pools { mint_a: String, mint_b: String },
    PoolHistory6Hour,
    PoolHistory24Hour,
    Trade24Hour,
    Volume24Hour,
    PoolVolumeHistory,
    PoolLiquidityHistory,
    TradingView,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        match item {
            API::Serum(route) => match route {
                Serum::Pairs => "/pairs".to_string(),
                Serum::Trades(market_name) => format!("{}{}", "/trades/", market_name),
                Serum::AddressTrade(market_address) => {
                    format!("{}{}", "/trades/address/", market_address)
                }
                Serum::AllRecentTrades => "/trades/all/recent".to_string(),
                Serum::Volumes(market_name) => format!("{}{}", "/volumes/", market_name),
                Serum::OrderBooks(market_name) => format!("{}{}", "/orderbooks/", market_name),
                Serum::HistoricalPrices(market_name) => format!("{}{}", "/candles/", market_name),
                Serum::Pools { mint_a, mint_b } => format!("{}{}/{}", "/pools/", mint_a, mint_b),
                Serum::PoolHistory6Hour => "/pools".to_string(),
                Serum::PoolHistory24Hour => "/pools-recent".to_string(),
                Serum::Trade24Hour => "/trades".to_string(),
                Serum::Volume24Hour => "/pools/volumes/recent".to_string(),
                Serum::PoolVolumeHistory => "/pools/volumes".to_string(),
                Serum::PoolLiquidityHistory => "/pools/liquidity".to_string(),
                Serum::TradingView => "/tv".to_string(),
            },
        }
    }
}
