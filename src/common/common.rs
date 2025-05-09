use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub version: String,
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: u16,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn Ok(body: String) -> Self {
        HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_message: "OK".to_string(),
            headers: HashMap::new(),
            body,
        }
    }
    
    pub fn NotFound() -> Self {
        HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: 404,
            status_message: "Not Found".to_string(),
            headers: HashMap::new(),
            body: "404 Not Found".to_string(),
        }
    }

    pub fn BadRequest() -> Self {
        HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: 400,
            status_message: "Bad Request".to_string(),
            headers: HashMap::new(),
            body: "400 Bad Request".to_string(),
        }
    }

    pub fn plain_text(&self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_message
        );
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}