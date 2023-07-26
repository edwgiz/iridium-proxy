use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Deref;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::Mutex;
use std::time::Duration;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use tracing::{debug, error, info, warn};

use Parameter::*;

const PASSWORD: &'static str = "C5E4";

static BUS: Lazy<SyncSender<TcpRequest>> = Lazy::new(subscribe);
static INTENSIVE_RECV_ATTEMPTS: Lazy<AtomicU32> = Lazy::new(|| AtomicU32::new(1));

#[derive(Debug)]
struct TcpRequest {
    request_payload: Vec<u8>,
    on_response: fn(&str),
}

pub(crate) fn init() {
    let _bus: &SyncSender<TcpRequest> = BUS.deref(); // provoke lazy initialization
    info!("Connection loop started");
}

fn subscribe() -> SyncSender<TcpRequest> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(64);
    tokio::task::spawn_blocking(move || {
        let mut buffer: [u8; 512] = [0; 512];
        let status_request: TcpRequest = TcpRequest {
            request_payload: format!("VSt07_{PASSWORD}").into_bytes(),
            on_response: on_vst07_response,
        };
        let tcp_requesting_pause = Duration::from_millis(50);
        loop {
            if let Ok(mut stream) = TcpStream::connect("192.168.1.217:1560") {
                loop {
                    let intensive_recv_attempts = INTENSIVE_RECV_ATTEMPTS.fetch_update(
                        Ordering::SeqCst, Ordering::SeqCst,
                        |x| Some(if x == 0 { 0 } else { x - 1 })).unwrap();
                    let recv_timeout = if intensive_recv_attempts > 0 { 500 } else { 30_000 };
                    let handled: bool = match receiver.recv_timeout(Duration::from_millis(recv_timeout)) {
                        Ok(request) => {
                            send_request(&mut buffer, &mut stream, &request)
                        }
                        Err(_timeout) => {
                            send_request(&mut buffer, &mut stream, &status_request)
                        }
                    };
                    if !handled {
                        break;
                    }
                    std::thread::sleep(tcp_requesting_pause);
                }
                stream.shutdown(std::net::Shutdown::Both).unwrap_or_default();
            }
            info!("TCP reconnect in 30 secs...");
            std::thread::sleep(Duration::from_secs(50));
        }
    });

    return sender;
}

fn send_request(buffer: &mut [u8; 512], stream: &mut TcpStream, request: &TcpRequest) -> bool {
    let request_size = request.request_payload.len();
    buffer[..request_size].copy_from_slice(&request.request_payload);
    debug!("intensivity {}, command {}", INTENSIVE_RECV_ATTEMPTS.load(Ordering::Relaxed), std::str::from_utf8(&buffer[..request_size]).unwrap_or_default());
    if stream.write_all(&buffer[..request_size]).is_ok() && stream.flush().is_ok() {
        if let Ok(response_size) = stream.read(buffer) {
            let response_handler = request.on_response;
            if let Ok(response_payload) = std::str::from_utf8(&buffer[0..response_size]) {
                response_handler(response_payload);
            } else {
                warn!("Incorrect utf8 string: {:x?}", &buffer[0..response_size]);
            }
        } else {
            warn!("TCP reading failed");
            return false;
        }
    } else {
        warn!("TCP writing failed");
        return false;
    }
    return true;
}

const VST07_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^VSt07_([0-9a-f]+)_([0-9a-f]+)_([0-9a-f]+)_[0-9a-f]+_([0-9a-f]+)").unwrap());

static VALUES: Lazy<Mutex<HashMap<&'static Parameter, String>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(10)));

#[derive(Eq, Hash, PartialEq, Debug)]
enum Parameter {
    PwrBtnState,
    UnitState,
    Tempr,
    SpeedTarget,
    SpeedFact,
}

fn on_vst07_response(line: &str) {
    match VST07_PATTERN.captures(line) {
        Some(captures) => {
            if let Ok(bits) = parse_hex_capture(&captures, 1) {
                let value: bool = bits & 0x1 == 1;
                let value: String = String::from(if value { '1' } else { '0' });
                store_value(&PwrBtnState, value);
            }
            if let Ok(bits) = parse_hex_capture(&captures, 2) {
                let value: u8 = (bits & 0x3) as u8;
                store_value(&UnitState, value.to_string());
            }
            if let Ok(bits) = parse_hex_capture(&captures, 3) {
                let value = i8::try_from(bits & 0xff).unwrap();
                store_value(&Tempr, value.to_string());
            }
            if let Ok(bits) = parse_hex_capture(&captures, 4) {
                {
                    let value = i8::try_from((bits >> 4) & 0xf).unwrap();
                    store_value(&SpeedTarget, value.to_string());
                }
                {
                    let value = i8::try_from((bits >> 8) & 0xff).unwrap();
                    store_value(&SpeedFact, value.to_string());
                }
            }
        }
        None => {
            warn!("Can't parse vst07 response: {line}");
        }
    }
}

fn store_value(parameter: &'static Parameter, value: String) {
    if let Some(old_value) = VALUES.lock().unwrap().insert(parameter, value.clone()) {
        if !old_value.eq(&value) {
            INTENSIVE_RECV_ATTEMPTS.store(30, Ordering::Relaxed);
            crate::websocket::broadcast(&format(parameter), &value);
            debug!("Parameter {:?}: {old_value} -> {value}", parameter);
        }
    } else {
        crate::websocket::broadcast(&format!("Breezart.{:?}", parameter), &value);
        debug!("Parameter {:?}: {value}", parameter);
    }
}

fn format(parameter: &'static Parameter) -> String {
    format!("Breezart.{:?}", parameter)
}

pub fn get_all_values() -> Vec<(String, String)> {
    let src = VALUES.lock().unwrap();
    let mut dst = Vec::with_capacity(src.len());
    for tuple in src.iter() {
        dst.push((format(tuple.0), tuple.1.clone()));
    }
    return dst;
}

fn parse_hex_capture(captures: &Captures, capture_index: usize) -> Result<u16, String> {
    return match captures.get(capture_index) {
        Some(capture) => {
            u16::from_str_radix(capture.as_str(), 16).map_err(|e| e.to_string())
        }
        None => {
            Err(format!("No capture at {} index", capture_index))
        }
    };
}


pub(crate) fn send_set(channel_name: &str, value: &str) -> bool {
     match channel_name {
        "Breezart.PwrBtn" => {
            if "1".eq(value) || "0".eq(value) {
                enqueue_req(format!("VWPwr_{PASSWORD}_{value}"));
            } else {
                warn!("Invalid Breezart.PwrBtn: {value}");
            }
        }
        "Breezart.SpeedTarget" => {
            match u8::from_str_radix(value, 10) {
                Ok(value) => {
                    if value <= 10 {
                        enqueue_req(format!("VWSpd_{PASSWORD}_{:X}", value));
                    }
                }
                Err(parse_err) => {
                    warn!("Invalid Breezart.SpeedTarget: {value}, {parse_err}");
                }
            }
        }
        &_ => {return false;}
    }
    return true;
}

fn enqueue_req(msg: String) {
    let tcp_request = TcpRequest {
        request_payload: msg.clone().into_bytes(),
        on_response: on_response_stub,
    };
    match BUS.try_send(tcp_request) {
        Ok(_) => {
            debug!("Queued: {}", msg);
        }
        Err(TrySendError::Disconnected(_tcp_request)) => {
            error!("Not queued: {}, Sending on a closed channel", msg)
        }
        Err(err) => {
            warn!("Not queued: {}, {}", msg, err);
        }
    }
}

fn on_response_stub(response_payload: &str) {
    if !response_payload.starts_with("OK_") {
        warn!("Unexpected response: {response_payload}");
    } else {
        debug!(response_payload);
        INTENSIVE_RECV_ATTEMPTS.store(30, Ordering::Relaxed);
    }
}
