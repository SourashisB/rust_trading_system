use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::models::asset::Asset;
use std::path::Path;

pub struct AssetRepository {
    db_path: Box<Path>,
}

impl AssetRepository {
    pub fn new(db_path: &Path) -> Self {
        AssetRepository {
            db_path: db_path.into(),
        }
    }

    pub fn add_asset(&self, asset: &Asset) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO assets (id, name, symbol, price) VALUES (?1, ?2, ?3, ?4)",
            params![
                asset.id.to_string(),
                asset.name,
                asset.symbol,
                asset.price
            ],
        )?;
        Ok(())
    }

    pub fn get_asset_by_id(&self, id: &Uuid) -> Result<Option<Asset>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM assets WHERE id = ?1")?;
        let asset_iter = stmt.query_map(params![id.to_string()], |row| {
            Ok(Asset {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                name: row.get(1)?,
                symbol: row.get(2)?,
                price: row.get(3)?,
            })
        })?;

        let asset = asset_iter.filter_map(Result::ok).next();
        Ok(asset)
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM assets")?;
        let asset_iter = stmt.query_map([], |row| {
            Ok(Asset {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                name: row.get(1)?,
                symbol: row.get(2)?,
                price: row.get(3)?,
            })
        })?;

        let assets: Result<Vec<Asset>, _> = asset_iter.collect();
        assets
    }

    pub fn update_asset_price(&self, id: &Uuid, new_price: f64) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "UPDATE assets SET price = ?1 WHERE id = ?2",
            params![new_price, id.to_string()],
        )?;
        Ok(())
    }
}