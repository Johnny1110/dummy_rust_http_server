
use std::error::Error;

use crate::common::{common::HttpRequest, common::HttpResponse};

pub fn greeting(http_req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    dbg!("Request: {:?}", http_req);
    let mut response = HttpResponse::Ok("Hello, World!".to_string());
    response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response.headers.insert("Content-Length".to_string(), response.body.len().to_string());
    response.headers.insert("Connection".to_string(), "close".to_string());
    response.headers.insert("Server".to_string(), "Rust HTTP Server".to_string());
    response.headers.insert("Date".to_string(), "Wed, 21 Oct 2015 07:28:00 GMT".to_string());
    response.headers.insert("Accept".to_string(), "text/html".to_string());
    response.headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
    response.headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());

    Ok(response)
}

pub fn mock_a_long_query(http_req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    dbg!("Request: {:?}", http_req);
    let mut response = HttpResponse::Ok("Hello, World! After a long query.".to_string());
    response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response.headers.insert("Content-Length".to_string(), response.body.len().to_string());
    response.headers.insert("Connection".to_string(), "close".to_string());
    response.headers.insert("Server".to_string(), "Rust HTTP Server".to_string());
    response.headers.insert("Date".to_string(), "Wed, 21 Oct 2015 07:28:00 GMT".to_string());
    response.headers.insert("Accept".to_string(), "text/html".to_string());
    response.headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
    response.headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());

    // Simulate a long query
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(response)
}