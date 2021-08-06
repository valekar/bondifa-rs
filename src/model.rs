//use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
//use std::fmt;
//use std::marker::PhantomData;
//use std::str::FromStr;
use std::vec::Vec;
//use void::Void;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub success: bool,
    pub data: Vec<String>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub success: bool,
    #[serde(with = "string_or_struct")]
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
    pub order_id: String,
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
pub struct NullData {
    pub success: String,
    pub data: String,
}

pub(crate) mod string_or_struct {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use std::fmt;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Serialize,
    {
        #[derive(Deserialize, Serialize)]
        #[serde(untagged)]
        enum StringOrStruct<T> {
            String(Option<String>),
            Struct(Vec<T>),
        }

        match StringOrStruct::deserialize(deserializer)? {
            StringOrStruct::String(_) => Ok(Vec::new()),
            StringOrStruct::Struct(i) => Ok(i),
        }
    }
}
