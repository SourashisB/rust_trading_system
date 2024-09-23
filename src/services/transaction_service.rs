use crate::models::transaction::{Transaction, TransactionType};
use crate::repositories::transaction_repository::TransactionRepository;
use crate::services::wallet_service::WalletService;
use crate::services::asset_service::AssetService;
use uuid::Uuid;

pub struct TransactionService {
    transaction_repository: TransactionRepository,
    wallet_service: WalletService,
    asset_service: AssetService,
}

impl TransactionService {
    pub fn new() -> Self {
        TransactionService {
            transaction_repository: TransactionRepository::new(),
            wallet_service: WalletService::new(),
            asset_service: AssetService::new(),
        }
    }

    pub fn create_transaction(
        &self,
        transaction_type: TransactionType,
        asset_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        amount: f64,
    ) -> Result<Transaction, String> {
        let asset = self.asset_service.get_asset_by_id(&asset_id).ok_or("Asset not found")?;
        let total_value = amount * asset.price;

        match transaction_type {
            TransactionType::Buy => {
                self.wallet_service.update_balance(&from_user_id, &asset_id, amount)?;
                self.wallet_service.update_balance(&to_user_id, &Uuid::nil(), total_value)?;
            }
            TransactionType::Sell => {
                self.wallet_service.update_balance(&from_user_id, &asset_id, -amount)?;
                self.wallet_service.update_balance(&to_user_id, &Uuid::nil(), -total_value)?;
            }
        }

        let transaction = Transaction::new(transaction_type, asset_id, from_user_id, to_user_id, amount);
        self.transaction_repository.add_transaction(transaction.clone())?;
        Ok(transaction)
    }

    pub fn get_transaction_by_id(&self, id: &Uuid) -> Option<Transaction> {
        self.transaction_repository.get_transaction_by_id(id)
    }

    pub fn get_transactions_by_user_id(&self, user_id: &Uuid) -> Vec<Transaction> {
        self.transaction_repository.get_transactions_by_user_id(user_id)
    }
}