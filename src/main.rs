#![allow(unused_imports)]
use std::env::args;
use std::fmt::write;
use std::str;
use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};

use kafka_starter_rust::api_versions::ApiVersions;
use kafka_starter_rust::constants::API_VERSIONS_REQUEST_API_KEY;
use kafka_starter_rust::RequestHeader;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                while _stream.peek(&mut [0; 1]).is_ok() {
                    handle_connection(&_stream);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let mut size: [u8; 4] = [0; 4];

    stream
        .read_exact(&mut size)
        .expect("Failed to read icoming request");

    let mut request_bytes: Vec<u8> = vec![0; u32::from_be_bytes(size) as usize];

    stream
        .read(&mut request_bytes)
        .expect("Unable to read request body");

    let headers: RequestHeader = RequestHeader::new(request_bytes);

    let mut res_bytes: Vec<u8> = vec![0];

    match headers.request_api_key {
        API_VERSIONS_REQUEST_API_KEY => {
            let response: ApiVersions = ApiVersions::new(&headers);
            res_bytes = response.serialize_to_bytes();
        }
        _ => println!("[DEBUG]: This type of request is not available yet."),
    }

    stream
        .write_all(&res_bytes)
        .expect("Error writing to TcpStream");
}
