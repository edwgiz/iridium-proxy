use core::result::Result;
use std::convert::Infallible;
use std::include_bytes;
use std::ops::Deref;

use const_format::formatcp;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use warp::Filter;

mod commons;
mod log;
mod websocket;


#[derive(RustEmbed)]
#[folder = "www/static/"]
struct StaticResources;

#[derive(RustEmbed)]
#[folder = "www/tls/"]
#[include = "*.pem"]
struct TlsResources;

//noinspection HttpUrlsUsage
const PROXY_URL_BASE: &'static str = formatcp!("http://{}/", commons::PROXY_HOST);
const PASSWORD: &'static [u8] = "edwgiz".as_bytes();
static PROXY_CLIENT: Lazy<reqwest::Client> = Lazy::new(proxy_client);

async fn call_proxy_login(body: hyper::body::Bytes) -> Result<hyper::Response<hyper::Body>, warp::Rejection> {
    let mut local_response = warp::http::Response::builder();
    let mut status_code = warp::http::status::StatusCode::NOT_ACCEPTABLE;

    if body.eq(PASSWORD) {
        let proxy_client = PROXY_CLIENT.deref();

        let remote_request = proxy_client
            .request(reqwest::Method::POST, PROXY_URL_BASE)
            .header(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap())
            .body("Password=2007&name=authform&Login=admin")
            .build()
            .map_err(commons::Error::Request)
            .map_err(warp::reject::custom)?;

        let remote_response = proxy_client
            .execute(remote_request)
            .await
            .map_err(commons::Error::Request)
            .map_err(warp::reject::custom)?;


        let set_cookie = remote_response
            .headers()
            .get(reqwest::header::SET_COOKIE);



        let success = set_cookie
            .map_or(false, |header_value| {
                return header_value.to_str().map_or(false, |str| {
                    return str.starts_with("ir-session-id=");
                });
            });

        if success {
            status_code = warp::http::status::StatusCode::OK;
            local_response = local_response.status(warp::http::status::StatusCode::OK);
            for (k, v) in remote_response.headers().iter() {
                local_response = local_response.header(k, v);
            }
        }
    }

    return local_response
        .status(status_code)
        .body(hyper::Body::empty())
        .map_err(commons::Error::Http)
        .map_err(warp::reject::custom);
}


async fn call_proxy(
    cookie: Option<warp::http::HeaderValue>,
    tail_path: warp::path::Tail,
    query: String,
) -> Result<warp::http::Response<hyper::Body>, warp::Rejection> {
    let mut remote_uri: String = PROXY_URL_BASE.to_string();
    remote_uri.push_str(tail_path.as_str());
    if !query.is_empty() {
        remote_uri.push('?');
        remote_uri.push_str(query.as_str());
    }

    let proxy_client = proxy_client();

    let mut remote_request_builder = proxy_client
        .request(reqwest::Method::GET, remote_uri);
    for v in cookie.iter() {
        remote_request_builder = remote_request_builder.header("cookie", v.clone());
    }
    let remote_request = remote_request_builder.build()
        .map_err(commons::Error::Request)
        .map_err(warp::reject::custom)?;

    let remote_response = proxy_client
        .execute(remote_request)
        .await
        .map_err(commons::Error::Request)
        .map_err(warp::reject::custom)?;

    let mut local_response = warp::http::Response::builder();
    for (k, v) in remote_response.headers().iter() {
        local_response = local_response.header(k, v);
    }
    return local_response
        .status(remote_response.status())
        .body(hyper::Body::wrap_stream(remote_response.bytes_stream()))
        .map_err(commons::Error::Http)
        .map_err(warp::reject::custom);
}

fn proxy_client() -> reqwest::Client {
    return reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Default reqwest client couldn't build");
}

pub async fn local_websocket_handler(cookie: Option<warp::http::HeaderValue>, ws: warp::ws::Ws) -> Result<impl warp::Reply, Infallible> {
    let reply: Box<dyn warp::Reply>;
    let authorized = call_proxy_ok(cookie).await;
    if authorized {
        reply = Box::new(ws.on_upgrade(move |local_socket| websocket::on_websocket_upgrade(local_socket)));
    } else {
        reply = Box::new(warp::http::StatusCode::NOT_ACCEPTABLE);
    }
    return Ok(reply);
}


async fn call_proxy_ok(cookie: Option<hyper::header::HeaderValue>) -> bool {
    let mut remote_uri: String = PROXY_URL_BASE.to_string();
    remote_uri.push_str("json/ok");

    let proxy_client = proxy_client();

    let mut remote_request_builder = proxy_client
        .request(reqwest::Method::GET, remote_uri);
    for v in cookie.iter() {
        remote_request_builder = remote_request_builder.header("cookie", v.clone());
    }
    let remote_request = remote_request_builder.build().unwrap();

    let remote_response_result = proxy_client
        .execute(remote_request)
        .await;

    return remote_response_result.map_or(false, |v| {
        v.status() == reqwest::StatusCode::OK
    });
}

fn main() {
    log::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    runtime.block_on(async {
        let proxy_http_route = warp::path("proxy")
            .and(warp::header::optional("cookie"))
            .and(warp::path::tail())
            .and(warp::query::raw())
            .and_then(call_proxy)
            .boxed();

        let proxy_login_http_route = warp::post()
            .and(warp::path("login"))
            .and(warp::body::bytes())
            .and_then(call_proxy_login)
            .boxed();

        let proxy_websocket_route = warp::path("proxy")
            .and(warp::header::optional("cookie"))
            .and(warp::ws())
            .and_then(local_websocket_handler)
            .boxed();

        let static_resources = warp_embed::embed(&StaticResources {});
        warp::serve(
            proxy_http_route
                .or(proxy_login_http_route)
                .or(static_resources)
                .or(proxy_websocket_route)
                .boxed())
            .tls()
            .cert(include_bytes!("../www/tls/cert.pem"))
            .key(include_bytes!("../www/tls/key.pem"))
            .run(([0, 0, 0, 0], 1443))
            .await;
    });
}
