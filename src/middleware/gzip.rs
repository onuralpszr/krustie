use std::io::Write;
use flate2::{ write::GzEncoder, Compression };

use crate::{ request::HttpRequest, response::HttpResponse, server::route_handler::HandlerResult };
use super::Middleware;

/// A middleware for compressing response body using gzip
///
/// # Example
///
/// ```rust
/// use krustie::{server::Server, middleware::gzip::GzipEncoder};
///
/// fn main() {
///   let mut server = Server::create();
///
///   server.use_handler(GzipEncoder);
/// }
///
#[derive(Debug)]
pub struct GzipEncoder;

impl GzipEncoder {
    fn encode(body: &Vec<u8>) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        if encoder.write_all(body.as_slice()).is_err() {
            return Err("Error while writing to encoder".to_string());
        }

        match encoder.finish() {
            Ok(compressed_bytes) => {
                return Ok(compressed_bytes);
            }
            Err(err) => {
                return Err(format!("{err}"));
            }
        }
    }
}

impl Middleware for GzipEncoder {
    fn middleware(&self, request: &HttpRequest, response: &mut HttpResponse) -> HandlerResult {
        let body = response.get_body_mut();

        if body.len() == 0 {
            return HandlerResult::Next;
        }

        match request.get_header("accept-encoding") {
            Some(str_encodings) => {
                let encodings = str_encodings
                    .split(",")
                    .map(|item| item.trim())
                    .collect::<Vec<&str>>();

                if encodings.contains(&"gzip") {
                    response.insert_header("Content-Encoding", "gzip");
                    let body = response.get_body_mut();

                    match Self::encode(body) {
                        Ok(compressed_bytes) => {
                            *body = compressed_bytes;
                        }
                        Err(err) => {
                            eprintln!("Error while compressing: {}", err);
                        }
                    }
                }
            }
            None => {}
        }

        return HandlerResult::Next;
    }
}
