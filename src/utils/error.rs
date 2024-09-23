use std::fmt;

#[derive(Debug)]
pub enum TradingError {
    UserNotFound,
    AssetNotFound,
    WalletNotFound,
    InsufficientBalance,
    InvalidTransaction,
    // Add more error types as needed
}

impl fmt::Display for TradingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TradingError::UserNotFound => write!(f, "User not found"),
            TradingError::AssetNotFound => write!(f, "Asset not found"),
            TradingError::WalletNotFound => write!(f, "Wallet not found"),
            TradingError::InsufficientBalance => write!(f, "Insufficient balance"),
            TradingError::InvalidTransaction => write!(f, "Invalid transaction"),
        }
    }
}

impl std::error::Error for TradingError {}