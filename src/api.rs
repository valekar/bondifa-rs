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
    Pools { mintA: String, mintB: String },
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
        String::from(match item {
            API::Serum(route) => match route {
                Serum::Pairs => "/pairs",
                Serum::Trades(market_name) => format!("{}{}", "/trades/", market_name),
                Serum::AddressTrade(market_address) => {
                    format!("{}{}", "/trades/address/", market_address)
                }
                Serum::AllRecentTrades => "/trades/all/recent",
                Serum::Volumes(market_name) => format!("{}{}", "/volumes/", market_name),
                Serum::OrderBooks(market_name) => format!("{}{}", "/orderbooks/", market_name),
                Serum::HistoricalPrices(market_name) => format!("{}{}", "/candles/", market_name),
                Serum::Pools { mintA, mintB } => format!("{}{}/{}", "/pools/", mintA, mintB),
                Serum::PoolHistory6Hour => "/pools",
                Serum::PoolHistory24Hour => "/pools-recent",
                Serum::Trade24Hour => "/trades",
                Serum::Volume24Hour => "/pools/volumes/recent",
                Serum::PoolVolumeHistory => "/pools/volumes",
                Serum::PoolLiquidityHistory => "/pools/liquidity",
                Serum::TradingView => "/tv",
            },
        })
    }
}
