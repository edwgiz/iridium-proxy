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
use tracing::{error, info, warn};
use url::Url;

//noinspection RsUnusedImport
const PROXY_WEBSOCKET_URL: &'static str = formatcp!("ws://{}/", crate::commons::PROXY_HOST);

static CROSSBEAM_MESSAGE_BUS: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(subscribe);

pub async fn websocket_unauthorized(mut local_socket: warp::filters::ws::WebSocket) {
    local_socket.send(
        warp::ws::Message::close_with(tungstenite::protocol::frame::coding::CloseCode::Policy, "unauthorized"))
        .await.unwrap_or_default();
    local_socket.close()
        .await.unwrap_or_default();
}

pub async fn on_websocket_upgrade(local_socket: warp::filters::ws::WebSocket) {
    let (mut local_ws_tx, mut local_ws_rx) = local_socket.split();

    let local_active = Arc::new(AtomicBool::new(true));
    let remote_active = Arc::clone(&local_active);

    if let Ok(chained_local_ws_tx) = read_remote_all(local_ws_tx).await {
        local_ws_tx = chained_local_ws_tx;
    } else {
        return;
    }

    let remote_read_task = subscribe_remote_websocket(local_ws_tx, remote_active);

    loop {
        match local_ws_rx.next().await {
            None => {
                info!("Client socket: No more messages");
                break;
            }
            Some(msg) => {
                match msg {
                    Ok(msg) => {
                        if msg.is_text() {
                            match msg.to_str() {
                                Ok(msg) => {
                                    if let Some((channel_name, value)) = msg.split_once(";") {
                                        if !crate::iridium::http_client::send_set(channel_name, value).await {
                                            warn!("Client socket: Can't pass message: {msg}");
                                            break;
                                        }
                                    } else {
                                        error!("Client socket: Unknown text message format: {msg}");
                                    }
                                }
                                Err(_) => {
                                    error!("Client socket: Incorrect text message")
                                }
                            }
                        } else if msg.is_close() {
                            info!("Client socket: Close message received");
                            break;
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }
    local_active.store(false, Ordering::Relaxed);
    remote_read_task.abort();
    remote_read_task.await.unwrap();
}

async fn read_remote_all(mut local_ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>) -> Result<SplitSink<warp::ws::WebSocket, warp::ws::Message>, String> {
    return match crate::iridium::http_client::read_all_tags().await {
        Ok(tags_body) => {
            for tag in tags_body.tags {
                let msg = format!("tag;{};{};{}", tag.id, tag.name, tag.value);
                if let Err(send_err) = local_ws_tx.send(warp::ws::Message::text(msg)).await {
                    return Err(send_err.to_string());
                }
            }
            Ok(local_ws_tx)
        }
        Err(err) => {
            Err(err)
        }
    }
}


fn subscribe_remote_websocket(mut local_ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>, remote_active: Arc<AtomicBool>) -> JoinHandle<bool> {
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
                info!("Iridium websocket connected");
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
                info!("Iridium websocket reconnect in 30 secs...");
                std::thread::sleep(Duration::from_secs(30));
            }
        }
    });
    return message_bus;
}
