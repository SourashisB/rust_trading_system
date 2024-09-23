use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::transaction::Transaction;

#[derive(Clone)]
pub struct TransactionRepository {
    transactions: Arc<Mutex<Vec<Transaction>>>,
}

impl TransactionRepository {
    pub fn new() -> Self {
        TransactionRepository {
            transactions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), String> {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
        Ok(())
    }

    pub fn get_transaction_by_id(&self, id: &Uuid) -> Option<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.iter().find(|t| t.id == *id).cloned()
    }

    pub fn get_transactions_by_user_id(&self, user_id: &Uuid) -> Vec<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions
            .iter()
            .filter(|t| t.from_user_id == *user_id || t.to_user_id == *user_id)
            .cloned()
            .collect()
    }
}