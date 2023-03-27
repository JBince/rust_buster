pub mod banner;
pub mod config;
pub mod scanner;
use reqwest::StatusCode;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const DEFAULT_STATUS_CODES: [StatusCode; 2] = [
    //2xx response codes
    StatusCode::OK,
    StatusCode::CREATED,
];
