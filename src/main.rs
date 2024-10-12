#![allow(unused_imports)]
use core::str;
use std::{
    collections::VecDeque,
    io::Write,
    net::{TcpListener, TcpStream},
    vec,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(&_stream);
                match _stream.shutdown(std::net::Shutdown::Both) {
                    Ok(()) => (),
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
    let correlation_id: u32 = 7;
    let response_size: [u8; 4] = [0, 0, 0, 4];
    let response: Vec<u8> = [response_size, correlation_id.to_be_bytes()].concat();
    match stream.write_all(&response) {
        Ok(()) => (),
        Err(_) => println!("Some error occured"),
    }
}
