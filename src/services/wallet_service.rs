use crate::models::wallet::Wallet;
use crate::repositories::wallet_repository::WalletRepository;
use uuid::Uuid;

pub struct WalletService {
    wallet_repository: WalletRepository,
}

impl WalletService {
    pub fn new() -> Self {
        WalletService {
            wallet_repository: WalletRepository::new(),
        }
    }

    pub fn create_wallet(&self, user_id: Uuid) -> Result<Wallet, String> {
        let wallet = Wallet::new(user_id);
        self.wallet_repository.add_wallet(wallet.clone())?;
        Ok(wallet)
    }

    pub fn get_wallet_by_user_id(&self, user_id: &Uuid) -> Option<Wallet> {
        self.wallet_repository.get_wallet_by_user_id(user_id)
    }

    pub fn update_balance(&self, user_id: &Uuid, asset_id: &Uuid, amount: f64) -> Result<(), String> {
        let mut wallet = self.get_wallet_by_user_id(user_id).ok_or("Wallet not found")?;
        let balance = wallet.balances.entry(*asset_id).or_insert(0.0);
        *balance += amount;
        self.wallet_repository.update_wallet(wallet)
    }
}