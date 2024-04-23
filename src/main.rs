
#![allow(dead_code)]
mod method; // Declare the method module first
mod server;
mod request;
mod query_string;
mod status_code; 
mod response;
mod website_handler;
// Declare the server module

use std::env;

// Now you can import items from the declared modules
use method::Method;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let delete = Method::DELETE;
    let put = Method::PUT;

    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path : {}",public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
    