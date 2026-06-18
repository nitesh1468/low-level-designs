#![allow(unused)]

use std::collections::HashMap;
use std::io::{Error,ErrorKind};


#[derive(Debug,Clone)]
struct HttpRequest {
    url: String,
    method: String,
    headers: HashMap<String,String>,
    body: String
}

impl HttpRequest {
    fn new() -> Self {
        Self {
            url: String::new(),
            method: String::new(),
            headers: HashMap::new(),
            body: String::new()
        }
    }
}

struct HttpRequestBuilder {
    request: HttpRequest
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self {
            request: HttpRequest::new()
        }
    }
    
    fn url(mut self,value:&str) -> Self {
        self.request.url = value.to_owned();
        self
    }

    fn method(mut self,value:&str) -> Self {
        self.request.method = value.to_owned();
        self
    }

    fn body(mut self,value:&str) -> Self {
        self.request.body = value.to_owned();
        self
    }

    fn headers(mut self,key:&str, value:&str) -> Self {
        self.request.headers.insert(key.to_owned(), value.to_owned());
        self
    }

    fn build(&self) -> Result<HttpRequest,Error> {
        if self.request.url.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "request url is required"));
        }
        Ok(self.request.clone())
    }


}

fn main() {
    let mut result = HttpRequestBuilder::new()
        .url("https://api.example.com")
        .method("post")
        .headers("Content-Type", "application/json")
        .body("{\"key\": \"value\"}")
        .build();

    match result {
        Ok(request) => {
            println!("{request:#?}");
        },
        Err(err) => {
            println!("{err}");
        } 
    }

}
