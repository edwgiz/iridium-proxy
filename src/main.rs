use core::result::Result;
use std::convert::Infallible;
use std::include_bytes;
use std::num::{NonZeroUsize, ParseIntError};
use std::sync::Arc;

use lru::LruCache;
use rand::RngCore;
use rust_embed::RustEmbed;
use tokio::sync::Mutex;
use warp::Filter;

mod commons;
mod log;
mod websocket;
mod iridium;
mod breezart;
mod knx;


#[derive(RustEmbed)]
#[folder = "www/static/"]
struct StaticResources;

#[derive(RustEmbed)]
#[folder = "www/tls/"]
#[include = "*.pem"]
struct TlsResources;

//noinspection HttpUrlsUsage
const WEB_PASSWORD: &'static [u8] = b"edwgiz";

async fn login(body: hyper::body::Bytes, auth_db: Arc<Mutex<LruCache<u64, u64>>>) -> Result<hyper::Response<hyper::Body>, warp::Rejection> {
    let mut local_response = warp::http::Response::builder();
    let mut status_code = warp::http::status::StatusCode::NOT_ACCEPTABLE;

    if body.eq(WEB_PASSWORD) {
        let now = u64::try_from(time::Instant::now().elapsed().whole_milliseconds()).unwrap();
        use rand::prelude::StdRng;
        use rand::prelude::SeedableRng;
        let session_id = StdRng::seed_from_u64(now).next_u64();
        match auth_db.lock().await {
            mut auth_db => {
                auth_db.push(session_id.clone(), now);
            }
        }
        let session_id = format!("{:X}", session_id);
        let cookie_value = format!("session_id={session_id}; Path=/; Max-Age=2147483647");
        local_response = local_response.header( warp::http::header::SET_COOKIE, cookie_value);

        status_code = warp::http::status::StatusCode::OK;
    }

    return local_response
        .status(status_code)
        .body(hyper::Body::empty())
        .map_err(commons::Error::Http)
        .map_err(warp::reject::custom);
}


pub async fn local_websocket_handler(session_id: Option<String>, auth_db: Arc<Mutex<LruCache<u64, u64>>>, ws: warp::ws::Ws) -> Result<impl warp::Reply, Infallible> {
    let mut auth_flag = false;
    if let Some(session_id) = session_id {
        if let Ok(session_id) = u64::from_str_radix(session_id.as_str(), 16) {
            match auth_db.lock().await {
                mut auth_db => {
                    auth_flag = auth_db.get(&session_id).is_some();
                }
            }
        }
    }
    let reply: Box<dyn warp::Reply>;
    if auth_flag {
        reply = Box::new(ws.on_upgrade(websocket::on_websocket_upgrade));
    } else {
        reply = Box::new(ws.on_upgrade(websocket::websocket_unauthorized));
    }
    return Ok(reply);
}


const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud19200,
    char_size: serial::Bits8,
    parity: serial::ParityEven,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    log::init();

    use serial;
    use serial::prelude::*;
    use tracing::{debug, error, info, warn};
    use std::time::Duration;
    use std::io::ErrorKind;
    use knx_rs::helper::hex_to_string;
    use std::io::Read;

/*    let mut port = serial::open("/dev/ttymxc1").expect("couldn't open serial port");
    port.configure(&SETTINGS)
        .expect("couldn't set serial settings");
    port.set_timeout(Duration::from_secs(10))
        .expect("couldn't set timeout");

*/
    info!("start reading bytes");

    use knx_rs::parser::parse_imi;


    let strs = [
        "BC 11 02 18 0C E1 00 80 25",
        "BC 11 05 18 02 E1 00 81 2D",
        "BC 11 05 19 02 E2 00 80 F7 D9",
        "BC 11 02 18 0C E1 00 81 24",
        "BC 11 05 18 02 E1 00 80 2C BC 11 05 19 02 E2 00 80 00 2E"
    ];
    /**
    y Ok(([37], ({ 1.1.2 }, { 1/8/12 }, (225, 128))))
    y Ok(([45], ({ 1.1.5 }, { 1/8/2 }, (225, 129))))
    y Ok(([247, 217], ({ 1.1.5 }, { 1/9/2 }, (226, 128))))
    y Ok(([36], ({ 1.1.2 }, { 1/8/12 }, (225, 129))))
    y Ok(([44, 188, 17, 5, 25, 2, 226, 0, 128, 0, 46], ({ 1.1.5 }, { 1/8/2 }, (225, 128))))
*/
    for str in strs {
        let x: Result<Vec<u8>, ParseIntError> = (0..str.len()).step_by(3).map(|i| u8::from_str_radix(&str[i..i + 2], 16)).collect();
        let vec = x.unwrap();
        let buf = vec.as_slice();

        info!("{:?}", knx::parse_tp(&buf));
    }


//    println!("{:?}",cemi);

    /*
    loop {
        let mut buf = [0; 24];
        match port.read(&mut buf) {
            Ok(nr) => {
                info!("{} -> {}", hex_to_string(&buf[0..nr]), nr);
            }
            Err(err) => {
                if err.kind() != ErrorKind::TimedOut {
                    info!(" result : {:?}", err)
                }
            }
        }
    }


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
                .tls()
                .cert(include_bytes!("../www/tls/cert.pem"))
                .key(include_bytes!("../www/tls/key.pem"))
                .run(([0, 0, 0, 0], 1443))
                .await;
        });
     */
}
