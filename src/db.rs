use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_db(db_path: &Path) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS assets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            symbol TEXT NOT NULL UNIQUE,
            price REAL NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallets (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL UNIQUE,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallet_balances (
            wallet_id TEXT NOT NULL,
            asset_id TEXT NOT NULL,
            balance REAL NOT NULL,
            PRIMARY KEY (wallet_id, asset_id),
            FOREIGN KEY (wallet_id) REFERENCES wallets (id),
            FOREIGN KEY (asset_id) REFERENCES assets (id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            transaction_type TEXT NOT NULL,
            asset_id TEXT NOT NULL,
            from_user_id TEXT NOT NULL,
            to_user_id TEXT NOT NULL,
            amount REAL NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (asset_id) REFERENCES assets (id),
            FOREIGN KEY (from_user_id) REFERENCES users (id),
            FOREIGN KEY (to_user_id) REFERENCES users (id)
        )",
        [],
    )?;

    Ok(())
}

pub fn get_db_connection(db_path: &Path) -> Result<Connection> {
    Connection::open(db_path)
}