#![allow(unused_imports)]
use std::fmt::write;
use std::str;
use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};

use kafka_starter_rust::api_versions::ResponseBody;
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
    let mut size: [u8; 4] = [0; 4];

    stream
        .read_exact(&mut size)
        .expect("Failed to read icoming request");

    let mut request_bytes: Vec<u8> = vec![0; u32::from_be_bytes(size) as usize];

    stream
        .read(&mut request_bytes)
        .expect("Unable to read request body");

    let headers: RequestHeader = RequestHeader::new(request_bytes);

    let response: ResponseBody = ResponseBody::new(&headers);

    let res_bytes: Vec<u8> = response.serialize_to_bytes();

    stream
        .write_all(&res_bytes)
        .expect("Error writing to TcpStream");
}
