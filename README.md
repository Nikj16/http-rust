# Rust HTTP Server

A lightweight, from-scratch HTTP/1.1 server implementation in Rust that serves static files. This project demonstrates low-level network programming, HTTP protocol handling, and secure file serving.

## Features

- ✅ HTTP/1.1 protocol support
- ✅ Static file serving
- ✅ Query string parsing
- ✅ Path traversal attack protection
- ✅ Custom request handling via trait system
- ✅ Zero external dependencies (pure Rust stdlib)

## Project Structure

```
http_server/
├── src/
│   ├── main.rs              # Entry point and server initialization
│   ├── http_server.rs       # Core TCP server and request handling
│   ├── website_handler.rs   # Static file handler implementation
│   └── http/
│       ├── mod.rs           # HTTP module exports
│       ├── request.rs       # HTTP request parsing
│       ├── response.rs      # HTTP response generation
│       ├── method.rs        # HTTP method enum (GET, POST, etc.)
│       ├── status_code.rs   # HTTP status codes
│       └── query_string.rs  # Query parameter parsing
└── public/                  # Static files directory
    ├── index.html
    ├── hello.html
    └── style.css
```

## How It Works

### 1. Server Initialization
The server binds to `127.0.0.1:8080` and enters an infinite loop accepting TCP connections.

### 2. Request Processing
For each incoming connection:
1. Read raw bytes from the TCP stream
2. Parse bytes into an HTTP `Request` object
3. Extract method, path, and query parameters
4. Pass request to the handler

### 3. File Serving
The `WebsiteHandler`:
- Maps URL paths to files in the `public/` directory
- Implements security checks to prevent directory traversal
- Returns appropriate status codes (200, 404, 400)

### 4. Response Generation
Responses are formatted as HTTP/1.1 messages and written back to the TCP stream.

## Getting Started

### Prerequisites
- Rust 1.70+ (edition 2024)
- Cargo

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd http_server

# Build the project
cargo build --release

# Run the server
cargo run
```

The server will start on `http://127.0.0.1:8080`

### Configuration

Set a custom public directory via environment variable:
```bash
PUBLIC_PATH=/path/to/your/public cargo run
```

## Usage Examples

### Serving Static Files
```bash
# Visit in browser or use curl
curl http://127.0.0.1:8080/           # Serves index.html
curl http://127.0.0.1:8080/hello      # Serves hello.html
curl http://127.0.0.1:8080/style.css  # Serves style.css
```

### Query String Parsing
The server can parse query parameters:
```
http://127.0.0.1:8080/search?name=rust&lang=en
```
Access via `request.query_string().get("name")`

## Security Features

### Path Traversal Protection
The server canonicalizes all file paths and ensures they stay within the public directory:
```rust
// Blocks attempts like: /../../../etc/passwd
if path.starts_with(&self.public_path) {
    // OK to serve
} else {
    println!("Attack attempted");
    None
}
```

## Future Improvements

### Short Term
- [ ] **HTTP Headers**: Parse and send proper Content-Type, Content-Length headers
- [ ] **POST Request Support**: Handle form data and file uploads
- [ ] **Better Error Pages**: Custom 404/500 HTML error pages
- [ ] **Logging**: Implement structured logging (with timestamps, request IDs)
- [ ] **Configuration File**: TOML/YAML config for port, paths, etc.

### Medium Term
- [ ] **Multi-threading**: Use thread pool to handle concurrent connections
- [ ] **Keep-Alive Support**: HTTP persistent connections
- [ ] **Compression**: Gzip/Brotli response compression
- [ ] **Range Requests**: Support partial content delivery (status 206)
- [ ] **MIME Type Detection**: Automatic Content-Type based on file extension
- [ ] **Directory Listings**: Auto-generate index pages for directories

### Long Term
- [ ] **HTTPS/TLS Support**: Secure connections via rustls
- [ ] **HTTP/2 Support**: Upgrade to HTTP/2 protocol
- [ ] **WebSocket Support**: Real-time bidirectional communication
- [ ] **Virtual Hosts**: Serve multiple domains from one server
- [ ] **Caching**: ETag and Last-Modified headers
- [ ] **Rate Limiting**: Prevent abuse/DoS attacks
- [ ] **Reverse Proxy**: Forward requests to backend services
- [ ] **CGI/FastCGI**: Execute dynamic scripts

### Performance Enhancements
- [ ] **Async I/O**: Migrate to tokio/async-std for better scalability
- [ ] **Zero-copy**: Use sendfile for efficient file transfers
- [ ] **Connection Pooling**: Reuse TCP connections
- [ ] **Buffer Tuning**: Optimize read/write buffer sizes

### Developer Experience
- [ ] **Unit Tests**: Comprehensive test coverage
- [ ] **Integration Tests**: End-to-end request/response testing
- [ ] **Benchmarks**: Performance profiling with criterion
- [ ] **CLI Arguments**: Accept port/address via command line
- [ ] **Hot Reload**: Auto-reload on file changes during development

## Contributing

This is a learning project, but contributions are welcome! Focus areas:
- Bug fixes
- Documentation improvements
- Performance optimizations
- Security enhancements

## License

MIT License - feel free to use this code for learning and projects.

## Acknowledgments

Built as a learning exercise to understand:
- TCP/IP networking in Rust
- HTTP protocol internals
- Systems programming concepts
- Secure file handling

---

**Note**: This is an educational project. For production use, consider mature HTTP servers like:
- [Actix-web](https://actix.rs/)
- [Rocket](https://rocket.rs/)
- [Axum](https://github.com/tokio-rs/axum)
