use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde_json::{json, Value};

use crate::services::{
    user_service::UserService,
    asset_service::AssetService,
    wallet_service::WalletService,
    transaction_service::TransactionService,
};

pub struct Server {
    user_service: UserService,
    asset_service: AssetService,
    wallet_service: WalletService,
    transaction_service: TransactionService,
}

impl Server {
    pub fn new() -> Self {
        Server {
            user_service: UserService::new(),
            asset_service: AssetService::new(),
            wallet_service: WalletService::new(),
            transaction_service: TransactionService::new(),
        }
    }

    pub fn run(&mut self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        println!("Server listening on {}", address);

        for stream in listener.incoming() {
            let mut stream = stream?;
            let server = self.clone();
            thread::spawn(move || {
                server.handle_client(stream);
            });
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while let Ok(size) = stream.read(&mut buffer) {
            if size == 0 {
                break;
            }
            let request = String::from_utf8_lossy(&buffer[..size]);
            let response = self.process_request(&request);
            let _ = stream.write_all(response.as_bytes());
        }
    }

    fn process_request(&self, request: &str) -> String {
        let request: Value = match serde_json::from_str(request) {
            Ok(req) => req,
            Err(_) => return json!({"status": "error", "message": "Invalid JSON"}).to_string(),
        };

        match request["command"].as_str() {
            Some("login") => self.handle_login(&request),
            Some("register") => self.handle_register(&request),
            Some("get_assets") => self.handle_get_assets(),
            Some("get_wallet") => self.handle_get_wallet(&request),
            Some("create_transaction") => self.handle_create_transaction(&request),
            _ => json!({"status": "error", "message": "Unknown command"}).to_string(),
        }
    }

    fn handle_login(&self, request: &Value) -> String {
        let username = request["username"].as_str().unwrap_or("");
        let password = request["password"].as_str().unwrap_or("");

        match self.user_service.authenticate(username, password) {
            Some(user) => json!({"status": "success", "user_id": user.id.to_string()}).to_string(),
            None => json!({"status": "error", "message": "Invalid credentials"}).to_string(),
        }
    }

    fn handle_register(&self, request: &Value) -> String {
        let username = request["username"].as_str().unwrap_or("");
        let password = request["password"].as_str().unwrap_or("");
        let role = request["role"].as_str().unwrap_or("user");

        match self.user_service.create_user(username.to_string(), password.to_string(), role.into()) {
            Ok(user) => json!({"status": "success", "user_id": user.id.to_string()}).to_string(),
            Err(e) => json!({"status": "error", "message": e}).to_string(),
        }
    }

    fn handle_get_assets(&self) -> String {
        let assets = self.asset_service.get_all_assets();
        json!({"status": "success", "assets": assets}).to_string()
    }

    fn handle_get_wallet(&self, request: &Value) -> String {
        let user_id = request["user_id"].as_str().unwrap_or("");
        match Uuid::parse_str(user_id) {
            Ok(user_id) => {
                match self.wallet_service.get_wallet_by_user_id(&user_id) {
                    Some(wallet) => json!({"status": "success", "wallet": wallet}).to_string(),
                    None => json!({"status": "error", "message": "Wallet not found"}).to_string(),
                }
            },
            Err(_) => json!({"status": "error", "message": "Invalid user ID"}).to_string(),
        }
    }

    fn handle_create_transaction(&self, request: &Value) -> String {
        // Implement transaction creation logic here
        // You'll need to extract transaction details from the request and call the appropriate service methods
        json!({"status": "success", "message": "Transaction created"}).to_string()
    }
}