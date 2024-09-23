use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::models::transaction::{Transaction, TransactionType};
use std::path::Path;
use chrono::{DateTime, Utc};

pub struct TransactionRepository {
    db_path: Box<Path>,
}

impl TransactionRepository {
    pub fn new(db_path: &Path) -> Self {
        TransactionRepository {
            db_path: db_path.into(),
        }
    }

    pub fn add_transaction(&self, transaction: &Transaction) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO transactions (id, transaction_type, asset_id, from_user_id, to_user_id, amount, timestamp) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                transaction.id.to_string(),
                format!("{:?}", transaction.transaction_type),
                transaction.asset_id.to_string(),
                transaction.from_user_id.to_string(),
                transaction.to_user_id.to_string(),
                transaction.amount,
                transaction.timestamp.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_transaction_by_id(&self, id: &Uuid) -> Result<Option<Transaction>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM transactions WHERE id = ?1")?;
        let transaction_iter = stmt.query_map(params![id.to_string()], |row| {
            Ok(Transaction {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                transaction_type: serde_json::from_str(&row.get::<_, String>(1)?).unwrap(),
                asset_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap(),
                from_user_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
                to_user_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap(),
                amount: row.get(5)?,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?).unwrap().with_timezone(&Utc),
            })
        })?;

        let transaction = transaction_iter.filter_map(Result::ok).next();
        Ok(transaction)
    }

    pub fn get_transactions_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Transaction>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM transactions WHERE from_user_id = ?1 OR to_user_id = ?1")?;
        let transaction_iter = stmt.query_map(params![user_id.to_string()], |row| {
            Ok(Transaction {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                transaction_type: serde_json::from_str(&row.get::<_, String>(1)?).unwrap(),
                asset_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap(),
                from_user_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
                to_user_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap(),
                amount: row.get(5)?,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?).unwrap().with_timezone(&Utc),
            })
        })?;

        let transactions: Result<Vec<Transaction>, _> = transaction_iter.collect();
        transactions
    }
}