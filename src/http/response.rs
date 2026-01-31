//! HTTP response generation and sending
//! 
//! Formats and writes HTTP responses to TCP streams.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use super::status_code::StatusCode;
#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response{
    /// Create a new HTTP response with the given status code and optional body
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Response{status_code, body}
    }
    
    /// Write the HTTP response to a stream (TCP connection)
    /// 
    /// Formats the response as: HTTP/1.1 {code} {reason}\r\n\n{body}
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{ 
        let body = match &self.body{
            Some(b) => b,
            None => ""
        };
        write!(stream,"HTTP/1.1 {} {}\r\n\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body)
    }
}
