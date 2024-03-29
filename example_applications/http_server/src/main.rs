#![allow(dead_code)]
use std::env;
use server::Server;
use web_handler::WebHandler;

mod server;
mod http;
mod web_handler;

fn main() {
    let bind_address = "127.0.0.1";
    let port = "8080";

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    
    println!("public path: {}", public_path);
    let server = Server::new(bind_address.into(), port.into());
    server.run(WebHandler::new(public_path));
}
