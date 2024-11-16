use super::constants::{
    API_VERSIONS_MAX_API_VERSION, API_VERSIONS_MIN_API_VERSION, API_VERSIONS_REQUEST_API_KEY,
};
use crate::RequestHeader;

pub struct ApiVersions<'a> {
    size: i32,
    correlation_id: i32,
    error_code: i16,
    num_of_keys: i8,
    api_keys: &'a [ApiKey],
    throttle_time_ms: i32,
    tag_buffer_len: i16,
}

struct ApiKey {
    api_key: i16,
    min_api_version: i16,
    max_api_version: i16,
}

impl<'a> ApiVersions<'a> {
    pub fn new(header: &RequestHeader) -> ApiVersions {
        let mut size: i32 = 13;
        let tag_buffer_len: i16 = 0;
        let num_of_keys: i8 = 2;
        let mut error_code: i16 = 0;

        size += ((num_of_keys - 1) * 6) as i32;

        if header.request_api_version < API_VERSIONS_MIN_API_VERSION
            || header.request_api_version > API_VERSIONS_MAX_API_VERSION
        {
            error_code = 35;
        }

        let api_keys: &[ApiKey; 1] = &[ApiKey {
            api_key: API_VERSIONS_REQUEST_API_KEY,
            min_api_version: API_VERSIONS_MIN_API_VERSION,
            max_api_version: API_VERSIONS_MAX_API_VERSION,
        }];

        let response = ApiVersions {
            size,
            correlation_id: header.correlation_id,
            num_of_keys,
            tag_buffer_len,
            error_code,
            throttle_time_ms: 0,
            api_keys,
        };

        response
    }

    pub fn serialize_to_bytes(&self) -> Vec<u8> {
        let mut serialized_response: Vec<u8> = Vec::new();

        serialized_response.extend_from_slice(&(self.size).to_be_bytes());
        serialized_response.extend_from_slice(&(self.correlation_id).to_be_bytes());
        serialized_response.extend_from_slice(&(self.error_code).to_be_bytes());
        serialized_response.extend_from_slice(&(self.num_of_keys).to_be_bytes());

        for key in self.api_keys {
            serialized_response.extend_from_slice(&(key.api_key).to_be_bytes());
            serialized_response.extend_from_slice(&(key.min_api_version).to_be_bytes());
            serialized_response.extend_from_slice(&(key.max_api_version).to_be_bytes());
        }

        serialized_response.extend_from_slice(&(self.tag_buffer_len).to_be_bytes());
        serialized_response.extend_from_slice(&(self.throttle_time_ms).to_be_bytes());

        serialized_response
    }
}
