use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use const_format::formatcp;
use crossbeam_channel;
use crossbeam_channel::{Receiver, Sender};
use futures::SinkExt;
use futures::stream::SplitSink;
use futures::StreamExt;
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use url::Url;

//noinspection RsUnusedImport
const PROXY_WEBSOCKET_URL: &'static str = formatcp!("ws://{}/", crate::commons::PROXY_HOST);

/// Additional runtime is required to don't block main tokio's one
pub static CROSSBEAM_MESSAGE_BUS: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(subscribe);


pub async fn on_websocket_upgrade(local_socket: warp::filters::ws::WebSocket) {
    let (local_ws_tx, mut local_ws_rx) = local_socket.split();

    let local_active = Arc::new(AtomicBool::new(true));
    let remote_active = Arc::clone(&local_active);

    let remote_read_task = read_remote_websocket(local_ws_tx, remote_active);

    loop {
        let msg_option = local_ws_rx.next().await;
        if msg_option.is_none() {
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
        let receiver = CROSSBEAM_MESSAGE_BUS.1.to_owned();
        while remote_active.load(Ordering::SeqCst) {
            let success: bool;
            match receiver.recv_timeout(Duration::from_secs(30)) {
                Ok(txt) => {
                    success = local_ws_tx.send(warp::ws::Message::text(txt)).await.is_ok();
                }
                Err(rte) => {
                    success = !rte.is_disconnected();
                }
            }
            if !success {
                remote_active.store(false, Ordering::SeqCst);
            }
        }
        local_ws_tx.close().await.unwrap_or_default();
        return true;
    };

    return tokio::spawn(future);
}


pub fn subscribe() -> (Sender<String>, Receiver<String>) {
    let message_bus = crossbeam_channel::bounded(64);
    let sender = message_bus.0.clone();
    tokio::spawn(async move {
        let url = Url::parse(PROXY_WEBSOCKET_URL).unwrap();
        loop {
            if let Ok((mut remote_socket, _)) = tungstenite::client::connect(url.clone()) {
                while let Ok(msg) = remote_socket.read_message() {
                    if msg.is_close() {
                        break;
                    }
                    if msg.is_text() {
                        if let Ok(txt) = msg.into_text() {
                            let _ = sender.try_send(txt).is_ok();
                        }
                    }
                }
                remote_socket.close(Some(tungstenite::protocol::CloseFrame {
                    code: tungstenite::protocol::frame::coding::CloseCode::Normal,
                    reason: Default::default(),
                })).unwrap_or_default();
            } else {
                std::thread::sleep(Duration::from_secs(30));
            }
        }
    });
    return message_bus;
}
