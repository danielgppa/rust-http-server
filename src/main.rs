#![allow(dead_code)]

use server::Server;

mod server;
mod http;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}