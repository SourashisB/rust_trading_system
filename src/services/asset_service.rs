use crate::models::asset::Asset;
use crate::repositories::asset_repository::AssetRepository;
use uuid::Uuid;

pub struct AssetService {
    asset_repository: AssetRepository,
}

impl AssetService {
    pub fn new() -> Self {
        AssetService {
            asset_repository: AssetRepository::new(),
        }
    }

    pub fn create_asset(&self, name: String, symbol: String, price: f64) -> Result<Asset, String> {
        let asset = Asset::new(name, symbol, price);
        self.asset_repository.add_asset(asset.clone())?;
        Ok(asset)
    }

    pub fn get_asset_by_id(&self, id: &Uuid) -> Option<Asset> {
        self.asset_repository.get_asset_by_id(id)
    }

    pub fn get_all_assets(&self) -> Vec<Asset> {
        self.asset_repository.get_all_assets()
    }

    pub fn update_asset_price(&self, id: &Uuid, new_price: f64) -> Result<(), String> {
        self.asset_repository.update_asset_price(id, new_price)
    }
}