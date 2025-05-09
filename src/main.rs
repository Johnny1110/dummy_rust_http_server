use std::{collections::HashMap, error::Error, io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, sync::Arc};

mod handler;
mod common;
mod route;
mod cp;

use common::common::{HttpRequest, HttpResponse};
use route::route::Route;
use cp::thread_pool::ThreadPool;

// Server configuration
const ADDRESS: &str = "127.0.0.1:7878";
const THREAD_POOL_SIZE: usize = 5;

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    println!("Listening on {}", ADDRESS);

    // Initialize router and register routes, Shared router.
    let router = Arc::new(Route::new());
    // Create a thread pool with a specified number of threads
    let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);
    println!("Thread pool created with {} threads", thread_pool.size);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone Arc for this job
            let router = Arc::clone(&router);
            thread_pool.exec(move || {
                handle_connection(&router, stream);
            });
            
        } else {
            eprintln!("Failed to accept connection");
        }
    }
}

fn handle_connection(router: &Arc<Route>, mut stream: TcpStream) {
    // parse the request
    match parse_http_request(&mut stream) {
        Ok(req) => {
            router.process_request(req, stream);  
        }
        Err(e) => {
            // Handle error, return a 400 Bad Request response
            eprintln!("Error parsing request: {}", e);
            let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    }

}

fn parse_http_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn Error>> {
    // Wrap the stream in a buffered reader for line-oriented reading.
    let mut reader = BufReader::new(stream);
    let mut first_line = String::new();

    // Read the request line: e.g., "GET /path HTTP/1.1"
    reader.read_line(&mut first_line)?;
    let parts: Vec<&str> = first_line.trim_end().split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid HTTP request line".into());
    }
    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    // Read headers
    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line = line.trim_end();
        if line.is_empty() {
            break; // Empty line indicates end of headers
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(
                name.trim().to_string(),
                value.trim().to_string(),
            );
        }
    }

    // Extract Host header or fallback
    let host = headers
        .get("Host")
        .cloned()
        .unwrap_or_default();

    // Determine body length
    let body = if let Some(len_str) = headers.get("Content-Length") {
        let len: usize = len_str.parse()?;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf)?
    } else {
        String::new()
    };

    Ok(HttpRequest {
        version,
        method,
        path,
        host,
        headers,
        body,
    })
}
