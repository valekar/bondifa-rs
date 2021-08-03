use crate::api::Rest;
use crate::api::API;
use crate::client::*;
use crate::errors::*;
use crate::model::*;

#[derive(Clone)]
pub struct Market {
    pub client: Client,
}

impl Market {
    pub fn get_all_pairs(&self) -> Result<Pair> {
        self.client.get(API::SerumRest(Rest::Pairs), None)
    }

    pub fn get_trades_for_market(&self, market_name: String) -> Result<Trade> {
        self.client
            .get(API::SerumRest(Rest::Trades(market_name)), None)
    }

    pub fn get_trades_for_market_address(&self, market_address: String) -> Result<Trade> {
        self.client
            .get(API::SerumRest(Rest::AddressTrade(market_address)), None)
    }

    pub fn get_all_24hr_trades(&self) -> Result<Trade> {
        self.client.get(API::SerumRest(Rest::AllRecentTrades), None)
    }

    pub fn get_volumes_for_market(&self, market_name: String) -> Result<Volume> {
        self.client
            .get(API::SerumRest(Rest::Volumes(market_name)), None)
    }

    pub fn get_order_books_for_market(&self, market_name: String) -> Result<OrderBook> {
        self.client
            .get(API::SerumRest(Rest::OrderBooks(market_name)), None)
    }
}
