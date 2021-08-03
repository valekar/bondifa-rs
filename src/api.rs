use crate::client::Client;
use crate::config::Config;
use crate::market::Market;

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
            },

            API::SerumWebSocket(route) => String::from(match route {
                WebSocket::Socket => "/ws",
                WebSocket::UnSubscribe => "/unsubscribe",
                WebSocket::Subscribe => "/subscribe",
            }),
        }
    }
}

pub trait Bonfida {
    fn new() -> Self;
    fn new_with_config(config: &Config) -> Self;
}

impl Bonfida for Market {
    fn new() -> Self {
        Self::new_with_config(&Config::default())
    }

    fn new_with_config(config: &Config) -> Self {
        Market {
            client: Client::new(config.rest_api_endpoint.clone()),
        }
    }
}
