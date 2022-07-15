use crate::msg::{HttpResponse, HttpRequest, HeaderField};
use ic_cdk::api::*;

// A common handlers
pub fn ok200() -> HttpResponse {
    HttpResponse {
        status_code: 200,
        headers: vec![HeaderField(String::from("content-type"), String::from("text/html"))],
        body: "Nothing to do".as_bytes().to_vec(),
        upgrade: Some(false),
    }
}

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse {
        status_code: 200,
        headers: vec![HeaderField(String::from("content-type"), String::from("text/plain"))],
        body: get_info().as_bytes().to_vec(),
        upgrade: Some(false),
    }
}

pub fn err404(req: HttpRequest) -> HttpResponse {
    HttpResponse {
        status_code: 404,
        headers: vec![],
        body: format!(
            "Nothing found at {}\n(but still, you reached the internet computer!)",
            req.url
        )
        .as_bytes()
        .to_vec(),
        upgrade: Some(false),
    }
}

pub fn get_info() -> String {
    format!(
        "
        这是部署在 Internet Computer 上的服务!\n 
        容器ID: {}\n
        容器剩余 cycle: {}\n
        ", id(),canister_balance())
}