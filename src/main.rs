#![allow(unused_imports)]
use core::str;
use std::{io::Write, net::{TcpListener, TcpStream}};

fn main() {
    println!("Logs from your program will appear here!");


    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(&_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: &TcpStream) {
    let response_size: u32 = 4; 
    let response_header: u32 = 7;
    let mut response: String = String::from("");
    response.push_str(&response_size.to_string());
    response.push_str(&response_header.to_string());
    match stream.write_all(response.as_bytes()) {
        Ok(()) => (),
        Err(_) => println!("Some error occured")
    }
}
