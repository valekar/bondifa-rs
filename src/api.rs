#[allow(clippy::all)]
pub enum API {
    SerumRest(Rest),
    SerumWebSocket(WebSocket),
}

pub enum Rest {
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

pub enum WebSocket {
    Socket,
    Subscribe,
    UnSubscribe,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        match item {
            API::SerumRest(route) => match route {
                Rest::Pairs => "/pairs".to_string(),
                Rest::Trades(market_name) => format!("{}{}", "/trades/", market_name),
                Rest::AddressTrade(market_address) => {
                    format!("{}{}", "/trades/address/", market_address)
                }
                Rest::AllRecentTrades => "/trades/all/recent".to_string(),
                Rest::Volumes(market_name) => format!("{}{}", "/volumes/", market_name),
                Rest::OrderBooks(market_name) => format!("{}{}", "/orderbooks/", market_name),
                Rest::HistoricalPrices(market_name) => format!("{}{}", "/candles/", market_name),
                Rest::Pools { mint_a, mint_b } => format!("{}{}/{}", "/pools/", mint_a, mint_b),
                Rest::PoolHistory6Hour => "/pools".to_string(),
                Rest::PoolHistory24Hour => "/pools-recent".to_string(),
                Rest::Trade24Hour => "/trades".to_string(),
                Rest::Volume24Hour => "/pools/volumes/recent".to_string(),
                Rest::PoolVolumeHistory => "/pools/volumes".to_string(),
                Rest::PoolLiquidityHistory => "/pools/liquidity".to_string(),
                Rest::TradingView => "/tv".to_string(),
            },

            API::SerumWebSocket(route) => String::from(match route {
                WebSocket::Socket => "/ws",
                WebSocket::UnSubscribe => "/unsubscribe",
                WebSocket::Subscribe => "/subscribe",
            }),
        }
    }
}
