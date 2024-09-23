use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::wallet::Wallet;

#[derive(Clone)]
pub struct WalletRepository {
    wallets: Arc<Mutex<Vec<Wallet>>>,
}

impl WalletRepository {
    pub fn new() -> Self {
        WalletRepository {
            wallets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_wallet(&self, wallet: Wallet) -> Result<(), String> {
        let mut wallets = self.wallets.lock().unwrap();
        if wallets.iter().any(|w| w.user_id == wallet.user_id) {
            return Err("Wallet already exists for this user".to_string());
        }
        wallets.push(wallet);
        Ok(())
    }

    pub fn get_wallet_by_user_id(&self, user_id: &Uuid) -> Option<Wallet> {
        let wallets = self.wallets.lock().unwrap();
        wallets.iter().find(|w| w.user_id == *user_id).cloned()
    }

    pub fn update_wallet(&self, wallet: Wallet) -> Result<(), String> {
        let mut wallets = self.wallets.lock().unwrap();
        if let Some(index) = wallets.iter().position(|w| w.id == wallet.id) {
            wallets[index] = wallet;
            Ok(())
        } else {
            Err("Wallet not found".to_string())
        }
    }
}