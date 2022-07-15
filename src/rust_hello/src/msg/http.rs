use candid::CandidType;
use serde::{Deserialize};


// The HTTP Gateway interfaces
// Taken from https://docs.rs/ic-utils/0.14.0/src/ic_utils/interfaces/http_request.rs.html
// and simplified (no streaming_strategy needed).

/// A key-value pair for a HTTP header.
#[derive(CandidType, Deserialize,Debug)]
pub struct HeaderField(pub String, pub String);

/// The important components of an HTTP request.
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    /// The HTTP method string.
    pub method: String,
    /// The URL that was visited.
    pub url: String,
    /// The request headers.
    pub headers: Vec<HeaderField>,
    /// The request body.
    // #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

impl  HttpRequest {
    pub fn new( url: String,  body: Vec<u8>) -> HttpRequest {
        HttpRequest { method: "".to_string(), url: url, headers:vec![], body: body }
    }
}

/// A HTTP response.
#[derive(CandidType,Debug)]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status_code: u16,
    /// The response header map.
    pub headers: Vec<HeaderField>,
    /// The response body.
    // #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    /// Whether the query call should be upgraded to an update call.
    pub upgrade: Option<bool>,
}

impl HttpResponse{
    pub fn new(body:String) -> HttpResponse{
        HttpResponse{
            status_code: 200,
            headers: vec![],
            body: body
            .as_bytes()
            .to_vec(),
            upgrade: Some(false),
        }
    }
}