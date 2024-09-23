use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balances: HashMap<Uuid, f64>,
}

impl Wallet {
    pub fn new(user_id: Uuid) -> Self {
        Wallet {
            id: Uuid::new_v4(),
            user_id,
            balances: HashMap::new(),
        }
    }
}