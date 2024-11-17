use super::constants::{
    ErrorCode, API_VERSIONS_MAX_API_VERSION, API_VERSIONS_MIN_API_VERSION, SUPPORTED_API,
};
use super::ApiKey;
use crate::RequestHeader;

pub struct ApiVersions<'a> {
    size: i32,
    correlation_id: i32,
    error_code: i16,
    api_keys: &'a [ApiKey],
    throttle_time_ms: i32,
    tag_buffer_len: i16,
}

impl<'a> ApiVersions<'a> {
    pub fn new(header: &RequestHeader) -> ApiVersions {
        let mut size: i32 = 13;
        let tag_buffer_len: i16 = 0;
        let mut error_code: i16 = 0;

        if header.request_api_version < API_VERSIONS_MIN_API_VERSION
            || header.request_api_version > API_VERSIONS_MAX_API_VERSION
        {
            error_code = ErrorCode::UnsupportedVersion as i16;
        }

        size += (SUPPORTED_API.len() * 6) as i32;

        let response = ApiVersions {
            size,
            correlation_id: header.correlation_id,
            tag_buffer_len,
            error_code,
            throttle_time_ms: 0,
            api_keys: SUPPORTED_API,
        };

        response
    }

    pub fn serialize_to_bytes(&self) -> Vec<u8> {
        let mut serialized_response: Vec<u8> = Vec::new();

        serialized_response.extend_from_slice(&(self.size).to_be_bytes());
        serialized_response.extend_from_slice(&(self.correlation_id).to_be_bytes());
        serialized_response.extend_from_slice(&(self.error_code).to_be_bytes());
        serialized_response.extend_from_slice(&((self.api_keys.len() + 1) as i8).to_be_bytes());

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
