use bytes::Buf;
use std::io::Read;

pub struct RequestHeader {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<String>,
}

impl RequestHeader {
    pub fn new(bytes: Vec<u8>) -> RequestHeader {
        // let mut size: [u8; 4] = [0; 4];
        let mut request_api_key: [u8; 2] = [0; 2];
        let mut request_api_version: [u8; 2] = [0; 2];
        let mut correlation_id: [u8; 4] = [0; 4];
        let mut client_id_length: [u8; 2] = [0; 2];
        let mut client_id: Option<String> = None;

        let mut reader = bytes.reader();

        // reader.read_exact(&mut size).unwrap();
        reader.read_exact(&mut request_api_key).unwrap();
        reader.read_exact(&mut request_api_version).unwrap();
        reader.read_exact(&mut correlation_id).unwrap();
        reader.read_exact(&mut client_id_length).unwrap();

        let client_id_len = i16::from_be_bytes(client_id_length);

        if client_id_len > 1 {
            let mut temp: String = String::new();
            reader.read_to_string(&mut temp).unwrap();
            client_id.get_or_insert(temp);
        }

        RequestHeader {
            client_id,
            correlation_id: i32::from_be_bytes(correlation_id),
            request_api_version: i16::from_be_bytes(request_api_version),
            request_api_key: i16::from_be_bytes(request_api_key),
        }
    }
}
