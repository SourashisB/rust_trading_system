#[cfg(test)]
mod tests {
    use trading_system::services::{
        user_service::UserService,
        asset_service::AssetService,
        wallet_service::WalletService,
        transaction_service::TransactionService,
    };
    use trading_system::models::user::UserRole;

    #[test]
    fn test_create_user_and_wallet() {
        let user_service = UserService::new();
        let wallet_service = WalletService::new();

        let user = user_service.create_user("testuser".to_string(), "password".to_string(), UserRole::User).unwrap();
        let wallet = wallet_service.create_wallet(user.id).unwrap();

        assert_eq!(wallet.user_id, user.id);
        assert!(wallet.balances.is_empty());
    }

    #[test]
    fn test_create_asset_and_transaction() {
        let user_service = UserService::new();
        let asset_service = AssetService::new();
        let wallet_service = WalletService::new();
        let transaction_service = TransactionService::new();

        let user1 = user_service.create_user("user1".to_string(), "password1".to_string(), UserRole::User).unwrap();
        let user2 = user_service.create_user("user2".to_string(), "password2".to_string(), UserRole::User).unwrap();

        wallet_service.create_wallet(user1.id).unwrap();
        wallet_service.create_wallet(user2.id).unwrap();

        let asset = asset_service.create_asset("Test Coin".to_string(), "TST".to_string(), 100.0).unwrap();

        let transaction = transaction_service.create_transaction(
            trading_system::models::transaction::TransactionType::Buy,
            asset.id,
            user1.id,
            user2.id,
            10.0,
        ).unwrap();

        assert_eq!(transaction.asset_id, asset.id);
        assert_eq!(transaction.from_user_id, user1.id);
        assert_eq!(transaction.to_user_id, user2.id);
        assert_eq!(transaction.amount, 10.0);
    }
}