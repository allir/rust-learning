#![allow(dead_code)]
use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;

fn main() {
    let get = Method::GET;

    let server = Server::new("172.0.0.1".into(), "8080".into());
    server.run();
}
