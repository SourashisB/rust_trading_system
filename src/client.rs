use std::net::TcpStream;
use std::io::{Read, Write};
use serde_json::json;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Client { stream })
    }

    pub fn send_request(&mut self, request: &str) -> std::io::Result<String> {
        self.stream.write_all(request.as_bytes())?;
        let mut response = String::new();
        self.stream.read_to_string(&mut response)?;
        Ok(response)
    }

    pub fn login(&mut self, username: &str, password: &str) -> std::io::Result<String> {
        let request = json!({
            "command": "login",
            "username": username,
            "password": password
        }).to_string();
        self.send_request(&request)
    }

    pub fn register(&mut self, username: &str, password: &str, role: &str) -> std::io::Result<String> {
        let request = json!({
            "command": "register",
            "username": username,
            "password": password,
            "role": role
        }).to_string();
        self.send_request(&request)
    }

    pub fn get_assets(&mut self) -> std::io::Result<String> {
        let request = json!({
            "command": "get_assets"
        }).to_string();
        self.send_request(&request)
    }

    pub fn get_wallet(&mut self, user_id: &str) -> std::io::Result<String> {
        let request = json!({
            "command": "get_wallet",
            "user_id": user_id
        }).to_string();
        self.send_request(&request)
    }

    pub fn create_transaction(&mut self, transaction_type: &str, asset_id: &str, from_user_id: &str, to_user_id: &str, amount: f64) -> std::io::Result<String> {
        let request = json!({
            "command": "create_transaction",
            "transaction_type": transaction_type,
            "asset_id": asset_id,
            "from_user_id": from_user_id,
            "to_user_id": to_user_id,
            "amount": amount
        }).to_string();
        self.send_request(&request)
    }
}