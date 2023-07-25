use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use const_format::formatcp;
use crossbeam_channel;
use crossbeam_channel::{Receiver, Sender};
use futures::SinkExt;
use futures::stream::SplitSink;
use futures::StreamExt;
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, Instrument, warn};
use url::Url;
use bus::Bus;

//noinspection RsUnusedImport
const PROXY_WEBSOCKET_URL: &'static str = formatcp!("ws://{}/", crate::commons::PROXY_HOST);

static BUS: Lazy<Mutex<Bus<String>>> = Lazy::new(|| Mutex::new(Bus::new(64)));

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
                                    debug!(msg);
                                    if let Some((channel_name, value)) = msg.split_once(";") {
                                        if crate::breezart::send_set(channel_name, value) {
                                        } else if !crate::iridium::http_client::send_set(channel_name, value).await {
                                            warn!("Client socket: Can't pass iridium message: {msg}");
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
    remote_read_task.await.unwrap_or_default();
}

async fn read_remote_all(mut local_ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>) -> Result<SplitSink<warp::ws::WebSocket, warp::ws::Message>, String> {
    return match crate::iridium::http_client::read_all_tags().await {
        Ok(tags_body) => {
            for tag in tags_body.tags {
                let msg = create_message(tag.id, &tag.name, &tag.value);
                if let Err(send_err) = local_ws_tx.send(warp::ws::Message::text(msg)).await {
                    return Err(send_err.to_string());
                }
            }
            for tuple in crate::breezart::get_all_values() {
                let msg = create_message(0, &tuple.0, &tuple.1);
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

fn create_message(id: u32, name: &String, value: &String) -> String {
    format!("tag;{id};{name};{value}")
}

pub(crate) fn broadcast(name: &String, value: &String) {
    broadcast_message(create_message(0, name, value));
}

fn broadcast_message(msg: String) {
    BUS.lock().unwrap().try_broadcast(
        msg
    ).unwrap_or_default();
}


fn subscribe_remote_websocket(mut local_ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>, remote_active: Arc<AtomicBool>) -> JoinHandle<bool> {
    return tokio::spawn(async move {
        let mut receiver = BUS.lock().unwrap().add_rx();
        while remote_active.load(Ordering::SeqCst) {
            let success: bool;
            match receiver.recv_timeout(Duration::from_secs(30)) {
                Ok(txt) => {
                    success = local_ws_tx.send(warp::ws::Message::text(txt)).await.is_ok();
                }
                Err(rte) => {
                    success = true;
                }
            }
            if !success {
                remote_active.store(false, Ordering::SeqCst);
            }
        }
        local_ws_tx.close().await.unwrap_or_default();
        return true;
    });
}


pub fn init() {
    tokio::spawn(async {
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
                            broadcast_message(txt);
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
}

use std::thread::{spawn, sleep, Thread};
use crossbeam_channel::{unbounded};


#[test]
fn it_works() {
    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();

    for i in 0..3 {
        let tx1 = tx1.clone();
        let rx2 = rx2.clone();
        spawn(move || consumer(i, tx1,rx2));
    }

    let vec_int:Vec<u64>=vec![1,2,3,4,5,6,7,8,9];
    spawn(move || producer(vec_int,rx1,tx2));

    thread::sleep(Duration::from_secs(100));
}

fn consumer(thread: i32, request: Sender<bool>, response: Receiver<u64>) {
    let mut receive_counter=3;
    loop {
        request.send(true).unwrap();
        let r =response.recv().unwrap();
        println!("Thread {} received {}",thread,r);
        receive_counter-=1;
        if receive_counter==0 {
            println!("Thread {} is done!", thread);
            break;
        }else {
            sleep(Duration::from_secs(r))
        }
    }
}

fn producer(mut vec_u64: Vec<u64>, request: Receiver<bool>, response: Sender<u64>) {
    loop{
        match request.try_recv(){
            Ok(_) => {
                let send_val= vec_u64.swap_remove(0);
                response.send(send_val).unwrap();
                if vec_u64.len()==0{
                    println!("Finishing producing");
                    break;
                }
            }
            _ => {

            }
        }
    }
}

