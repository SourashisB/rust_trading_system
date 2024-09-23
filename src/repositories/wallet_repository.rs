use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::models::wallet::Wallet;
use std::path::Path;
use std::collections::HashMap;

pub struct WalletRepository {
    db_path: Box<Path>,
}

impl WalletRepository {
    pub fn new(db_path: &Path) -> Self {
        WalletRepository {
            db_path: db_path.into(),
        }
    }

    pub fn add_wallet(&self, wallet: &Wallet) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO wallets (id, user_id) VALUES (?1, ?2)",
            params![wallet.id.to_string(), wallet.user_id.to_string()],
        )?;

        for (asset_id, balance) in &wallet.balances {
            tx.execute(
                "INSERT INTO wallet_balances (wallet_id, asset_id, balance) VALUES (?1, ?2, ?3)",
                params![wallet.id.to_string(), asset_id.to_string(), balance],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_wallet_by_user_id(&self, user_id: &Uuid) -> Result<Option<Wallet>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT id FROM wallets WHERE user_id = ?1")?;
        let wallet_id: Option<String> = stmt.query_row(params![user_id.to_string()], |row| row.get(0)).optional()?;

        if let Some(wallet_id) = wallet_id {
            let mut stmt = conn.prepare("SELECT asset_id, balance FROM wallet_balances WHERE wallet_id = ?1")?;
            let balances_iter = stmt.query_map(params![wallet_id], |row| {
                Ok((
                    Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    row.get::<_, f64>(1)?,
                ))
            })?;

            let balances: HashMap<Uuid, f64> = balances_iter.filter_map(Result::ok).collect();

            Ok(Some(Wallet {
                id: Uuid::parse_str(&wallet_id).unwrap(),
                user_id: *user_id,
                balances,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_wallet(&self, wallet: &Wallet) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let tx = conn.transaction()?;

        tx.execute(
            "DELETE FROM wallet_balances WHERE wallet_id = ?1",
            params![wallet.id.to_string()],
        )?;

        for (asset_id, balance) in &wallet.balances {
            tx.execute(
                "INSERT INTO wallet_balances (wallet_id, asset_id, balance) VALUES (?1, ?2, ?3)",
                params![wallet.id.to_string(), asset_id.to_string(), balance],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}