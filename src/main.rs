mod models;
mod repositories;
mod services;
mod utils;
mod server;
mod client;
mod db;

use std::env;
use std::path::Path;
use server::Server;
use client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <server|client> [address]", args[0]);
        return Ok(());
    }

    let mode = &args[1];
    let address = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");

    let db_path = Path::new("trading_system.db");
    db::init_db(db_path)?;

    match mode.as_str() {
        "server" => {
            let mut server = Server::new(db_path);
            if let Err(e) = server.run(address).await {
                eprintln!("Server error: {}", e);
            }
        }
        "client" => {
            match Client::new(address) {
                Ok(mut client) => run_client(&mut client).await?,
                Err(e) => eprintln!("Failed to connect to server: {}", e),
            }
        }
        _ => println!("Invalid mode. Use 'server' or 'client'."),
    }

    Ok(())
}

async fn run_client(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    // Client interaction logic (as provided in the previous response)
    Ok(())
}