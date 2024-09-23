mod models;
mod repositories;
mod services;
mod utils;
mod server;
mod client;

use std::env;
use server::Server;
use client::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <server|client> [address]", args[0]);
        return;
    }

    let mode = &args[1];
    let address = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");

    match mode.as_str() {
        "server" => {
            let mut server = Server::new();
            if let Err(e) = server.run(address) {
                eprintln!("Server error: {}", e);
            }
        }
        "client" => {
            match Client::new(address) {
                Ok(mut client) => run_client(&mut client),
                Err(e) => eprintln!("Failed to connect to server: {}", e),
            }
        }
        _ => println!("Invalid mode. Use 'server' or 'client'."),
    }
}

fn run_client(client: &mut Client) {
    // Client interaction logic (as provided in the previous response)
}