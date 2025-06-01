use std::vec;
use tokio::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use kafka_starter_rust::api_versions::ApiVersions;
use kafka_starter_rust::constants::{API_VERSIONS_REQUEST_API_KEY, FETCH_REQUEST_API_KEY};
use kafka_starter_rust::RequestHeader;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut size = [0; 4];
        println!("[DEBUG]: Reading first four bytes to get size of request");

        if let Err(e) = stream.read_exact(&mut size).await {
            eprintln!("[ERROR]: Connection closed or read failed {:?}", e);
            break;
        }

        println!("[DEBUG]: Size of request - {}", i32::from_be_bytes(size));

        let mut request_bytes: Vec<u8> = vec![0; i32::from_be_bytes(size) as usize];

        stream
            .read_exact(&mut request_bytes)
            .await
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
            .write(&res_bytes)
            .await
            .expect("Error writing to TcpStream");

        stream.flush().await.unwrap();

        println!("[DEBUG]: Write successfull.");
    }
}
