use crate::RequestHeader;

pub struct Fetch {
    size: i32,
    correlation_id: i32,
    throttle_time_ms: i32,
    error_code: i16,
    session_id: i32,
    responses: Vec<Response>,
    total_partitions: u16,
    total_aborted_transactions: u16,
}

struct Response {
    topic_id: [u8; 16],
    partitions: Vec<Partition>,
    num_of_paritions: u16,
}

struct Partition {
    partition_index: i32,
    error_code: i16,
    high_watermark: i64,
    last_stable_offset: i64,
    log_start_offset: i64,
    aborted_transactions: Vec<AbortedTransaction>,
    preffered_read_replica: i32,
    num_of_aborted_transactions: u16,
}

struct AbortedTransaction {
    producer_id: i64,
    first_offset: i64,
}

impl Fetch {
    pub fn new(header: &RequestHeader) -> Fetch {
        let size: i32 = 16;
        let correlation_id: i32 = header.correlation_id;

        Fetch {
            size,
            correlation_id,
            throttle_time_ms: 0,
            error_code: 0,
            session_id: 0,
            responses: Vec::new(),
            total_partitions: 0,
            total_aborted_transactions: 0,
        }
    }

    //pub fn serialize_to_bytes(&mut self) -> Vec<u8> {
    //    let mut serialized_bytes: Vec<u8> = Vec::new();

    //    self.size += (self.total_aborted_transactions * 16) as i32
    //        + (self.total_partitions * 36) as i32
    //        + (self.responses.len() * 16) as i32;

    //    serialized_bytes.extend_from_slice(&(self.size).to_be_bytes());
    //    serialized_bytes.extend_from_slice(&(self.correlation_id).to_be_bytes());
    //    serialized_bytes.extend_from_slice(&(self.throttle_time_ms).to_be_bytes());
    //    serialized_bytes.extend_from_slice(&(self.error_code).to_be_bytes());
    //    serialized_bytes.extend_from_slice(&(self.session_id).to_be_bytes());

    //    //size of responses array
    //    serialized_bytes.extend_from_slice(&((self.responses.len() + 1) as i8).to_be_bytes());

    //    for response in self.responses {
    //        serialized_bytes.
    //    }

    //    serialized_bytes
    //}
}
