use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::asset::Asset;

#[derive(Clone)]
pub struct AssetRepository {
    assets: Arc<Mutex<Vec<Asset>>>,
}

impl AssetRepository {
    pub fn new() -> Self {
        AssetRepository {
            assets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_asset(&self, asset: Asset) -> Result<(), String> {
        let mut assets = self.assets.lock().unwrap();
        if assets.iter().any(|a| a.symbol == asset.symbol) {
            return Err("Asset symbol already exists".to_string());
        }
        assets.push(asset);
        Ok(())
    }

    pub fn get_asset_by_id(&self, id: &Uuid) -> Option<Asset> {
        let assets = self.assets.lock().unwrap();
        assets.iter().find(|a| a.id == *id).cloned()
    }

    pub fn get_all_assets(&self) -> Vec<Asset> {
        let assets = self.assets.lock().unwrap();
        assets.clone()
    }

    pub fn update_asset_price(&self, id: &Uuid, new_price: f64) -> Result<(), String> {
        let mut assets = self.assets.lock().unwrap();
        if let Some(asset) = assets.iter_mut().find(|a| a.id == *id) {
            asset.price = new_price;
            Ok(())
        } else {
            Err("Asset not found".to_string())
        }
    }
}