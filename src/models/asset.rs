use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub price: f64,
}

impl Asset {
    pub fn new(name: String, symbol: String, price: f64) -> Self {
        Asset {
            id: Uuid::new_v4(),
            name,
            symbol,
            price,
        }
    }
}