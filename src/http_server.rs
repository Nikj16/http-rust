//! Core HTTP server implementation
//! 
//! Handles TCP connections, request parsing, and response generation.

use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request,Response,StatusCode, ParseError};

/// Trait for handling HTTP requests
/// 
/// Implement this trait to define custom request handling logic
pub trait Handler{
    /// Process a valid HTTP request and return a response
    fn handle_request(&mut self, request: &Request)-> Response;
    
    /// Handle malformed requests (returns 400 Bad Request by default)
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        // Log the parsing error to console
        println!("Failed to parse request{}", e);
        // Return a 400 Bad Request response with no body
        Response::new(StatusCode::BadRequest, None)
    }
}

/// HTTP Server that listens for connections and processes requests
pub struct HttpServer{
    address: String, // The address to bind to (e.g., "127.0.0.1:8080")
}

impl HttpServer{
    /// Create a new HTTP server bound to the specified address
    pub fn new(address: String) -> Self{
        Self{ address }
    }
    
    /// Start the server and begin accepting connections
    /// 
    /// This method runs indefinitely, processing incoming requests
    /// using the provided handler implementation.
    pub fn run(self, mut handler: impl Handler) {
        println!("Running the sever with address {}", self.address);
        // Bind to the TCP address - panics if address is already in use
        let listener = TcpListener::bind(&self.address).unwrap(); 
        
        // Main server loop - accepts and processes connections
        loop {
            // Accept blocks until a new connection arrives
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("New connection from: {}", addr);
                    // Allocate a 1KB buffer to read request data
                    let mut buffer = [0; 1024];
                    
                    // Read request data from the TCP stream
                    match stream.read(&mut buffer){
                        Ok(size)=>{
                            // Log the received data for debugging
                            println!("Received {} bytes: {:?}", size, String::from_utf8_lossy(&buffer[..size]));
                            
                            // Parse the request and generate response
                            // Try to parse the buffer into a Request struct
                            let response = match Request::try_from(&buffer[..]){
                                // If parsing succeeds, pass the request to the handler
                                Ok(request)=> handler.handle_request(&request),
                                // If parsing fails, handle the error (returns 400)
                                Err(e)=> handler.handle_bad_request(&e)
                            };

                            // Send response back to client
                            // Write the formatted HTTP response to the TCP stream
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send repsonse: {}", e);
                            }
                        },
                        Err(e) =>{
                            // Log I/O errors when reading from the stream
                            println!("Failed to read from connection: {}", e);
                        },
                    }
                },
                Err(e) => {
                    // Log errors when accepting new connections
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
