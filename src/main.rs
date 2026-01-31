//! A simple HTTP server implementation in Rust
//! 
//! This server serves static files from a public directory and handles basic HTTP requests.

#! [allow(dead_code)]

use http_server::HttpServer;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;
mod http_server;
mod http;
mod website_handler;

fn main() {
    // Determine the public directory path (can be overridden via PUBLIC_PATH env var)
    let default_path= format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path is {}", public_path);
    
    // Create and run the HTTP server on localhost:8080
    let server = HttpServer::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}