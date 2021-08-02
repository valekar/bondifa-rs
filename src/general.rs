use crate::api::Rest;
use crate::api::API;
use crate::client::*;
use crate::errors::*;
use crate::model::*;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    pub fn get_all_pairs(&self) -> Result<Pair> {
        self.client.get(API::SerumRest(Rest::Pairs), None)
    }
}
