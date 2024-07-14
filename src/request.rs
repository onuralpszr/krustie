//! This module contains the `Request` struct and its implementation.
//!
//! It has public methods to get the headers, add and get local variables
//!
//! All request parameters are non-mutable except for the *local* variables. Local variables can be
//! used to store data that can be defined in a *middleware* and accessed from the other
//! *middlewares* or *controllers*.

use std::{ collections::HashMap, fmt::{ Debug, Display, Formatter, Result as fResult } };

pub use body_type::BodyType;

use self::{ http_method::HttpMethod, request_line::RequestLine };

pub mod http_method;
pub mod body_type;

pub(crate) mod parser;
mod request_line;

/// Represents the HTTP request
pub struct Request {
    request: RequestLine,
    headers: HashMap<String, String>,
    body: BodyType,
    locals: HashMap<String, String>,
}

impl Request {
    /// Returns the reference of the Request headers as a HashMap
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ request::Request, response::Response};
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let headers = request.get_headers();
    ///   let content_type = headers.get("content-type");
    /// }
    /// ```
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the value of the header key
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ request::Request, response::Response};
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let content_type = request.get_header("content-type");
    /// }
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Returns the body of the HTTP request
    /// 
    /// The body can be of type `Text`, `Json`, `Form` or `None`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response, request::BodyType };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   match request.get_body() {
    ///     BodyType::Text(body) => {
    ///       // Do something with the body
    ///     },
    ///     BodyType::Json(json) => {
    ///      // Do something with the json
    ///     },
    ///     _ => {
    ///      // Do something else
    ///     }
    ///   }
    /// }
    /// 
    pub fn get_body(&self) -> &BodyType {
        &self.body
    }

    /// Adds a local variable to the http request
    ///
    /// `Local` variables can be used to store data that can be defined in a *middleware* and accessed in the *controller*
    pub fn add_local(&mut self, key: &str, value: &str) {
        self.locals.insert(key.to_string(), value.to_string());
    }

    /// Returns the value of the local variable
    ///
    /// `Local` variables can be used to store data that can be defined in a *middleware* and accessed in the *controller*
    pub fn get_local(&self, key: &str) -> Option<&String> {
        self.locals.get(key)
    }

    /// Returns the method of the HTTP request
    pub(crate) fn get_method(&self) -> &HttpMethod {
        self.request.get_method()
    }

    /// Returns the path of the HTTP request
    pub(crate) fn get_path_array(&self) -> &Vec<String> {
        &self.request.get_path_array()
    }

    fn header_parser() -> impl Fn(&String) -> Option<(String, String)> {
        |line: &String| {
            let header_line: Vec<&str> = line.split(':').collect();

            if header_line.len() == 2 {
                let key = header_line[0].trim().to_lowercase().to_string();
                let value = header_line[1].trim().to_string();

                Some((key, value))
            } else {
                None
            }
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            request: RequestLine::new("GET", "/", "HTTP/1.1").expect(
                "Failed to create default RequestLine"
            ),
            headers: HashMap::new(),
            body: BodyType::None,
            locals: HashMap::new(),
        }
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        let body = match &self.body {
            BodyType::Text(body) => format!("{:?}", body),
            BodyType::Json(json) => format!("{:?}", json),
            BodyType::Form(form) => format!("{:?}", form),
            BodyType::None => "None".to_string(),
        };

        write!(f, "Request Line: {}\r\n Headers: {}\r\n Body: {}", self.request, headers, body)
    }
}

#[derive(Debug)]
/// Represents an error that occurs when parsing an HTTP request
pub struct ParseHttpRequestError;

impl Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "Failed to parse HTTP request")
    }
}
