use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::Duration;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use tokio::runtime::Runtime;
use tracing::{debug, info, warn};

use Parameter::*;

const PASSWORD: &'static [u8] = b"_C5E4";

static REQUEST_BUS: Lazy<(crossbeam_channel::Sender<TcpRequest>, crossbeam_channel::Receiver<TcpRequest>)> = Lazy::new(|| crossbeam_channel::bounded(64));


struct TcpRequest {
    request_payload: Vec<u8>,
    on_response: fn(&str),
}

pub(crate) fn init(runtime: &Runtime) {
    runtime.spawn_blocking(connection_loop);
    info!("Connection loop started")
}

fn connection_loop() {
    let request_receiver = REQUEST_BUS.1.clone();
    let mut recv_timeout_in_sec: u64 = 1;
    let mut buffer: [u8; 512] = [0; 512];
    let status_request: TcpRequest = TcpRequest {
        request_payload: vst07_request(),
        on_response: on_vst07_response,
    };
    loop {
        if let Ok(mut stream) = TcpStream::connect("192.168.1.217:1560") {
            loop {
                let handled: bool = match request_receiver.recv_timeout(Duration::from_secs(recv_timeout_in_sec)) {
                    Ok(request) => {
                        recv_timeout_in_sec = 1;
                        on_request(&mut buffer, &mut stream, &request)
                    }
                    Err(_timeout) => {
                        recv_timeout_in_sec = u64::min(60, recv_timeout_in_sec * 2);
                        on_request(&mut buffer, &mut stream, &status_request)
                    }
                };
                if !handled {
                    break;
                }
            }
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }
        info!("TCP reconnect in 30 secs...");
        std::thread::sleep(Duration::from_secs(30));
    }
}

fn on_request(buffer: &mut [u8; 512], stream: &mut TcpStream, request: &TcpRequest) -> bool {
    let request_size = request.request_payload.len();
    buffer[..request_size].copy_from_slice(&request.request_payload);
    println!("{}", std::str::from_utf8(&buffer[..request_size]).unwrap_or_default());
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

pub fn vst07_request() -> Vec<u8> {
    return [b"VSt07", PASSWORD].concat();
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
            crate::websocket::send(&format(parameter), &value);
            debug!("Parameter {:?}: {old_value} -> {value}", parameter);
        }
    } else {
        crate::websocket::send(&format!("Breezart.{:?}", parameter), &value);
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

#[test]
fn it_works() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    REQUEST_BUS.0.clone().send(TcpRequest {
        request_payload: vst07_request(),
        on_response: |response_payload| {
            println!("{response_payload}");
        },
    }).unwrap_or_default();

    runtime.block_on(async { init(&runtime) });
    //   init();
}