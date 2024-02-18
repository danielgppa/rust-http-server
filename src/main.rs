#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run(WebsiteHandler);
}