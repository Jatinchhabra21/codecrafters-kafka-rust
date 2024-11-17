use super::ApiKey;

pub const API_VERSIONS_MIN_API_VERSION: i16 = 0;
pub const API_VERSIONS_MAX_API_VERSION: i16 = 4;
pub const API_VERSIONS_REQUEST_API_KEY: i16 = 18;

pub enum ErrorCode {
    UnsupportedVersion = 35,
}

pub const SUPPORTED_API: &[ApiKey; 1] = &[ApiKey {
    // APIVERSIONS request
    min_api_version: API_VERSIONS_MIN_API_VERSION,
    max_api_version: API_VERSIONS_MAX_API_VERSION,
    api_key: API_VERSIONS_REQUEST_API_KEY,
}];
