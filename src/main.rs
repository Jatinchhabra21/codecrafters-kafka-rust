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

fn parse_request_headers(mut stream: &TcpStream) -> RequestHeader {
    // read bytes that represent length
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length);

    // declare and initialize byte arrays for necessary fields
    let mut request_api_version: [u8; 2] = [0; 2];
    let mut correlation_id: [u8; 4] = [0; 4];
    let mut request_api_key: [u8; 2] = [0; 2];

    // read bytes into those byte arrays
    stream.read_exact(&mut request_api_key);
    stream.read_exact(&mut request_api_version);
    stream.read_exact(&mut correlation_id);

    // initialize RequestHeader enum
    RequestHeader {
        request_api_key: i16::from_be_bytes(request_api_key),
        request_api_version: i16::from_be_bytes(request_api_version),
        correlation_id: i32::from_be_bytes(correlation_id),
        client_id: None,
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let headers: RequestHeader = parse_request_headers(stream);
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

    stream.write_all(&size.to_be_bytes()); // size of resposne
    stream.write_all(&correlation_id.to_be_bytes()); // correlation id as bytes in big endian
    stream.write_all(&error_code.to_be_bytes()); // error code as bytes in big endian
    stream.write_all(&num_of_api_keys.to_be_bytes());
    stream.write_all(&api_key.to_be_bytes());
    stream.write_all(&min_api_version.to_be_bytes());
    stream.write_all(&max_api_version.to_be_bytes());
    stream.write_all(&tag_buffer_length.to_be_bytes());
    stream.write_all(&throttle_time_ms.to_be_bytes());
}
