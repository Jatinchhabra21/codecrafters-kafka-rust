#![allow(unused_imports)]
use std::str;
use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};

use kafka_starter_rust::RequestHeader;

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

fn handle_connection(mut stream: &TcpStream) {
    let mut header_bytes: Vec<u8> = Vec::new();
    stream.read_to_end(&mut header_bytes).unwrap();
    let headers: RequestHeader = RequestHeader::new(header_bytes);
    let size: i32 = 19;
    let correlation_id: i32 = headers.correlation_id;
    let mut error_code: i16 = 0;
    let num_of_api_keys: i8 = 2;
    let api_key: i16 = headers.request_api_key;
    let min_api_version: i16 = 0;
    let max_api_version: i16 = 4;
    let throttle_time_ms: i32 = 0;
    let tag_buffer_length: i16 = 0;

    // check if api version is invalid for ApiVersions request with key 18
    if headers.request_api_key == 18
        && (headers.request_api_version < 0 || headers.request_api_version > 4)
    {
        error_code = 35;
    }

    stream.write_all(&size.to_be_bytes()).unwrap(); // size of resposne
    stream.write_all(&correlation_id.to_be_bytes()).unwrap(); // correlation id as bytes in big endian
    stream.write_all(&error_code.to_be_bytes()).unwrap(); // error code as bytes in big endian
    stream.write_all(&num_of_api_keys.to_be_bytes()).unwrap();
    stream.write_all(&api_key.to_be_bytes()).unwrap();
    stream.write_all(&min_api_version.to_be_bytes()).unwrap();
    stream.write_all(&max_api_version.to_be_bytes()).unwrap();
    stream.write_all(&tag_buffer_length.to_be_bytes()).unwrap();
    stream.write_all(&throttle_time_ms.to_be_bytes()).unwrap();
}
