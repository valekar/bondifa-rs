use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pair {
    success: bool,
    data: Vec<String>,
}
