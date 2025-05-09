use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

use crate::handler::handler::greeting;
use crate::handler::handler::mock_a_long_query;
use crate::HttpRequest;
use crate::HttpResponse;

// Every handler is a function that takes a HttpRequest and returns an HttpResponse or an error.
// We use `Fn` (not `FnOnce`) so handlers can be stored in a map and invoked multiple times.
type Handler = Box<dyn Fn(HttpRequest) -> Result<HttpResponse, Box<dyn Error>> + Send + Sync>;
pub struct Route {
    // key: method_path
    // value: handler
    handler_map: HashMap<String, Handler>,
}

impl Route {
    pub fn new() -> Self {
        let mut handler_map: HashMap<String, Handler> = HashMap::new();
        // Add a handler for GET /hello; your `greeting` fn must match `fn(HttpRequest) -> Result<HttpResponse, _>`
        handler_map.insert("GET /hello".to_string(), Box::new(greeting));
        handler_map.insert("GET /long-query".to_string(), Box::new(mock_a_long_query));
        Route { handler_map }
    
    }

    /// Processes an incoming HTTP request by looking up and invoking the matching handler.
    pub fn process_request(&self, http_req: HttpRequest, mut stream: TcpStream) {
        // Check if the request method and path match any handler
        let key = format!("{} {}", http_req.method, http_req.path);
        
        if let Some(handler) = self.handler_map.get(&key) {
            match handler(http_req) {
                Ok(response) => {
                    let response_str = response.plain_text();
                    let _ = stream.write_all(response_str.as_bytes());
                }
                Err(e) => {
                    eprintln!("Error handling request: {}", e);
                    let _ = stream.write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n");
                }
            }
        } else {
            // No handler found: return 404 Not Found
            let response = HttpResponse::NotFound();
            let response_str = response.plain_text();
            let _ = stream.write_all(response_str.as_bytes());
        }
    }
}