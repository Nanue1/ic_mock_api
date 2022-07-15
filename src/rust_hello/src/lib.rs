pub mod msg;
pub mod util;
pub mod market;

use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use market::mock;
use msg::{HttpRequest, HttpResponse};
use util::http_common_handlers::{index, err404};

#[update]
fn http_request_update(req: HttpRequest) -> HttpResponse {
    dispatch(req)
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse{
    dispatch(req)
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
pub struct LoginRequest{
    username: String,
    password: String,
}

#[query]
fn login(req:LoginRequest) -> bool{
    if req.username == "manue1"{
        return true
    }
    false
}



fn dispatch(req: HttpRequest) -> HttpResponse {
    let uri = req.url.clone();
    match uri.strip_prefix("/v1/api/") {
        Some(sub) => router(sub, req),
        None => {
            if req.url == "/" {
                index(req)
            } else {
                err404(req)
            }
        }
    }
}

fn router(sub: &str,req:HttpRequest) -> HttpResponse {
    if sub == "greet" {
        return handle_greet(req)
    }
    if sub == "login" {
        return handle_login(req)
    }
    if sub == "dt_market_source" {
        return handle_dt_market_source(req)
    }
    err404(req)
}

fn handle_login(req: HttpRequest) -> HttpResponse {
    let body = String::from_utf8(req.body).unwrap();
    let req:LoginRequest = serde_json::from_str(body.as_str()).unwrap();
    let resp = serde_json::to_string(&login(req)).unwrap();
    HttpResponse::new(resp)
}

fn handle_greet(req: HttpRequest) -> HttpResponse {
    let name = String::from_utf8(req.body).unwrap();
    HttpResponse::new(greet(name))
}

fn handle_dt_market_source(_req: HttpRequest) -> HttpResponse {
    HttpResponse::new(mock::dt_market_source())
}