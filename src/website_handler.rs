//! Static file server handler
//! 
//! Serves HTML files and other static content from a public directory.

use super::http_server::Handler;
use crate::http::{Request, Response, StatusCode, Method};
use std::fs;

/// Handles requests by serving static files from a public directory
pub struct WebsiteHandler{
    public_path: String
}

impl WebsiteHandler{
    /// Create a new website handler with the specified public directory
    pub fn new(public_path: String)-> Self{
        Self{ public_path}
    }
    
    /// Read a file from the public directory
    /// 
    /// Includes path traversal protection to prevent accessing files
    /// outside the public directory.
    fn read_file(&self, file_addr: &str)-> Option<String>{
        let path = format!("{}/{}", self.public_path, file_addr);
        
        // Canonicalize to resolve .. and symlinks, preventing directory traversal attacks
        match fs::canonicalize(path){
            Ok(path) =>{
                // Security check: ensure resolved path is still within public_path
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                } else{
                    println!("Attack attempted {}", file_addr);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response{
        // Only handle GET requests
        match request.method(){
            Method::GET=> match request.path(){
                // Route: root path serves index.html
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                // Route: explicit hello page
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                // Route: try to serve any other requested file
                path => match self.read_file(path){
                    Some(contents)=> Response::new(StatusCode::Ok,Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                },
            },
            // Return 404 for non-GET methods
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
