#![allow(unused_imports)]
use std::str;
use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};

struct RequestHeader {
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
    client_id: Option<String>,
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(&_stream);
                match _stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => (),
                    Err(_) => println!("Some error occured when closing the connection"),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_bytes_to_uint(byte_array: &[u8]) -> usize {
    let mut result: usize = 0;
    let last_element_idx = byte_array.len() - 1;
    let idx = 0;

    for byte in byte_array {
        result = result + (*byte as usize) * (last_element_idx - idx) * 256;
    }

    result
}

// debug show function
fn show(bs: &[u8]) -> () {
    println!("{}", String::from_utf8_lossy(bs).into_owned());
}

fn parse_request_headers(mut stream: &TcpStream) -> RequestHeader {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length);
    let mut request_api_version: [u8; 2] = [0; 2];
    let mut correlation_id: [u8; 4] = [0; 4];
    let mut request_api_key: [u8; 2] = [0; 2];

    stream.read_exact(&mut request_api_key);
    stream.read_exact(&mut request_api_version);
    stream.read_exact(&mut correlation_id);

    RequestHeader {
        request_api_key: i16::from_be_bytes(request_api_key),
        request_api_version: i16::from_be_bytes(request_api_version),
        correlation_id: i32::from_be_bytes(correlation_id),
        client_id: None,
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let headers: RequestHeader = parse_request_headers(stream);
    let correlation_id: i32 = headers.correlation_id;
    let response_size: [u8; 4] = [0, 0, 0, 4];
    let response: Vec<u8> = [response_size, correlation_id.to_be_bytes()].concat();
    match stream.write_all(&response) {
        Ok(_) => (),
        Err(_) => println!("Some error occured"),
    }
}
