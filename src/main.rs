use std::vec;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use kafka_starter_rust::api_versions::ApiVersions;
use kafka_starter_rust::constants::{API_VERSIONS_REQUEST_API_KEY, FETCH_REQUEST_API_KEY};
use kafka_starter_rust::RequestHeader;

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                assert_eq!(stream.peek(&mut [0; 4]).await.is_ok(), true);
                let handler = tokio::spawn(async {
                    handle_connection(stream).await;
                });
                let _ = handler.await;
            }
            Err(error) => println!("Error occured: {:?}", error),
        };
    }
}

async fn handle_connection(mut stream: TcpStream) -> TcpStream {
    let mut size = [0; 4];
    println!("[DEBUG]: Reading first four bytes to get size of request");
    stream
        .try_read(&mut size)
        .expect("Failed to read incoming request");

    println!("[DEBUG]: Size of request - {}", i32::from_be_bytes(size));

    let mut request_bytes: Vec<u8> = vec![0; i32::from_be_bytes(size) as usize];

    stream
        .try_read(&mut request_bytes)
        .expect("Unable to read request body");

    println!("[DEBUG]: Request bytes - {:?}", request_bytes);

    let headers: RequestHeader = RequestHeader::new(request_bytes);

    let mut res_bytes: Vec<u8> = vec![0];

    match headers.request_api_key {
        API_VERSIONS_REQUEST_API_KEY => {
            let response: ApiVersions = ApiVersions::new(&headers);
            res_bytes = response.serialize_to_bytes();
        }
        _ => println!("[DEBUG]: This type of request is not available yet."),
    }

    println!("[DEBUG]: Writing to stream - {:?}", res_bytes);

    stream
        .write_all(&res_bytes)
        .await
        .expect("Error writing to TcpStream");

    println!("[DEBUG]: Write successfull.");

    stream
}
