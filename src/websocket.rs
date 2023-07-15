use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use const_format::formatcp;
use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use url::Url;

use crate::commons::PROXY_HOST;

const PROXY_WEBSOCKET_URL: &'static str = formatcp!("ws://{}/", PROXY_HOST);

static RUNTIME: Lazy<Runtime> = Lazy::new(|| tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap());

pub async fn on_websocket_upgrade(local_socket: warp::filters::ws::WebSocket) {
    let (mut local_ws_tx, mut local_ws_rx) = local_socket.split();

    let local_active = Arc::new(AtomicBool::new(true));
    let remote_active = Arc::clone(&local_active);

    let remote_read_task = read_remote_websocket(local_ws_tx, remote_active);

    loop {
        let msg_option = local_ws_rx.next().await;
        if(msg_option.is_none()) {
            break;
        }
        let msg_result = msg_option.unwrap();
        if msg_result.is_ok() {
            let message = msg_result.unwrap();
            if message.is_close() {
                break;
            }
        } else {
            break;
        }
    }
    local_active.store(false, Ordering::Relaxed);
    remote_read_task.abort();
    remote_read_task.await.unwrap();
    return;
}

fn read_remote_websocket(mut local_ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>, remote_active: Arc<AtomicBool>) -> JoinHandle<bool> {
    let future = async move {
        let url = Url::parse(PROXY_WEBSOCKET_URL).unwrap();
        let (mut remote_socket, _) =
            tungstenite::client::connect(url).unwrap();
        while remote_active.load(Ordering::Relaxed) {
            let result = remote_socket.read_message();
            if result.is_err() {
                break;
            }
            let msg = result.expect("Assert having message");
            if msg.is_close() {
                break;
            }
            if msg.is_text() {
                let txt = msg.into_text();
                if txt.is_ok() {
                    let success = local_ws_tx.send(warp::ws::Message::text(txt.unwrap())).await.is_ok();
                    if !success {
                        break;
                    }
                }
            }
        }
        remote_socket.close(Some(tungstenite::protocol::CloseFrame {
            code: tungstenite::protocol::frame::coding::CloseCode::Normal,
            reason: Default::default(),
        })).unwrap_or_default();
        local_ws_tx.close().await.unwrap_or_default();
        return true;
    };

    return RUNTIME.spawn(future);
}
