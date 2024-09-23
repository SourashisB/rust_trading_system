use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub asset_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Buy,
    Sell,
}

impl Transaction {
    pub fn new(
        transaction_type: TransactionType,
        asset_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        amount: f64,
    ) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            transaction_type,
            asset_id,
            from_user_id,
            to_user_id,
            amount,
            timestamp: Utc::now(),
        }
    }
}