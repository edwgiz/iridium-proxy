use core::result::Result;
use lru::LruCache;
use rand::RngCore;
use rust_embed::RustEmbed;
use std::convert::Infallible;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use warp::Filter;


mod commons;
mod log;
mod websocket;
mod iridium;
mod breezart;

#[derive(RustEmbed)]
#[folder = "www/static/"]
struct StaticResources;

#[derive(RustEmbed)]
#[folder = "www/tls/"]
#[include = "*.pem"]
struct TlsResources;

//noinspection HttpUrlsUsage
const WEB_PASSWORD: &[u8] = b"edwgiz";

async fn login(body: hyper::body::Bytes, auth_db: Arc<Mutex<LruCache<u64, u64>>>) -> Result<warp::hyper::Response<warp::hyper::Body>, warp::Rejection> {
    let mut local_response = warp::http::Response::builder();
    let mut status_code = warp::http::status::StatusCode::NOT_ACCEPTABLE;

    if body.eq(WEB_PASSWORD) {
        let now = u64::try_from(Instant::now().elapsed().as_millis()).unwrap();
        use rand::prelude::StdRng;
        use rand::prelude::SeedableRng;
        let session_id = StdRng::seed_from_u64(now).next_u64();
        let mut auth_db = auth_db.lock().await;
        auth_db.push(session_id, now);
        let session_id = format!("{:X}", session_id);
        let cookie_value = format!("session_id={session_id}; Path=/; Max-Age=2147483647");
        local_response = local_response.header( warp::http::header::SET_COOKIE, cookie_value);

        status_code = warp::http::status::StatusCode::OK;
    }

    local_response
        .status(status_code)
        .body(warp::hyper::Body::empty())
        .map_err(commons::Error::Http)
        .map_err(warp::reject::custom)
}


pub async fn local_websocket_handler(session_id: Option<String>, auth_db: Arc<Mutex<LruCache<u64, u64>>>, ws: warp::ws::Ws) -> Result<impl warp::Reply, Infallible> {
    let mut auth_flag = false;
    if let Some(session_id) = session_id {
        if let Ok(session_id) = u64::from_str_radix(session_id.as_str(), 16) {
            let mut auth_db = auth_db.lock().await;
            auth_flag = auth_db.get(&session_id).is_some();
        }
    }
    let reply: Box<dyn warp::Reply> = if auth_flag {
        Box::new(ws.on_upgrade(websocket::on_websocket_upgrade))
    } else {
        Box::new(ws.on_upgrade(websocket::websocket_unauthorized))
    };
    Ok(reply)
}


fn main() {
    log::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    runtime.block_on(async {
        breezart::init();
        websocket::init();

        let mut auth_cache: LruCache<u64, u64> = LruCache::new(NonZeroUsize::new(32).unwrap());
        auth_cache.push(u64::from_str_radix("F9681A64D3301861", 16).unwrap(), u64::MAX);
        let auth_cache = Arc::new(Mutex::new(auth_cache));
        let auth_cache = warp::any().map(move || Arc::clone(&auth_cache));

        let login_route = warp::post()
            .and(warp::path("login"))
            .and(warp::body::bytes())
            .and(auth_cache.clone())
            .and_then(login)
            .boxed();

        let proxy_websocket_route = warp::path("proxy")
            .and(warp::cookie::optional("session_id"))
            .and(auth_cache.clone())
            .and(warp::ws())
            .and_then(local_websocket_handler)
            .boxed();

        let static_resources = warp_embed::embed(&StaticResources {});
        warp::serve(
            login_route
                .or(static_resources)
                .or(proxy_websocket_route)
                .boxed())
/*            .tls()
            .cert(include_bytes!("../www/tls/cert.pem"))
            .key(include_bytes!("../www/tls/key.pem"))
*/            .run(([0, 0, 0, 0], 1443))
            .await;
    });
}
