pub mod api_versions;
pub mod constants;

pub struct ApiKey {
    pub api_key: i16,
    pub min_api_version: i16,
    pub max_api_version: i16,
}
