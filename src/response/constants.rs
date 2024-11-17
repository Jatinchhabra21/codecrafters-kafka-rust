use super::ApiKey;

pub const API_VERSIONS_MIN_API_VERSION: i16 = 0;
pub const API_VERSIONS_MAX_API_VERSION: i16 = 4;
pub const API_VERSIONS_REQUEST_API_KEY: i16 = 18;
pub const FETCH_MIN_API_VERSION: i16 = 0;
pub const FETCH_MAX_API_VERSION: i16 = 16;
pub const FETCH_REQUEST_API_KEY: i16 = 1;

pub enum ErrorCode {
    UnsupportedVersion = 35,
}

pub const SUPPORTED_API: &[ApiKey; 2] = &[
    // API VERSIONS REQUEST
    ApiKey {
        min_api_version: API_VERSIONS_MIN_API_VERSION,
        max_api_version: API_VERSIONS_MAX_API_VERSION,
        api_key: API_VERSIONS_REQUEST_API_KEY,
    },
    // FETCH REQUEST
    ApiKey {
        max_api_version: FETCH_MAX_API_VERSION,
        min_api_version: FETCH_MIN_API_VERSION,
        api_key: FETCH_REQUEST_API_KEY,
    },
];
