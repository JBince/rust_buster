pub mod banner;
pub mod config;
pub mod scanner;
use reqwest::StatusCode;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const ALL_STATUS_CODES: [StatusCode; 60] = [
    // all 1XX response codes
    StatusCode::CONTINUE,
    StatusCode::SWITCHING_PROTOCOLS,
    StatusCode::PROCESSING,
    // all 2XX response codes
    StatusCode::OK,
    StatusCode::CREATED,
    StatusCode::ACCEPTED,
    StatusCode::NON_AUTHORITATIVE_INFORMATION,
    StatusCode::NO_CONTENT,
    StatusCode::RESET_CONTENT,
    StatusCode::PARTIAL_CONTENT,
    StatusCode::MULTI_STATUS,
    StatusCode::ALREADY_REPORTED,
    StatusCode::IM_USED,
    // all 3XX response codes
    StatusCode::MULTIPLE_CHOICES,
    StatusCode::MOVED_PERMANENTLY,
    StatusCode::FOUND,
    StatusCode::SEE_OTHER,
    StatusCode::NOT_MODIFIED,
    StatusCode::USE_PROXY,
    StatusCode::TEMPORARY_REDIRECT,
    StatusCode::PERMANENT_REDIRECT,
    // all 4XX response codes
    StatusCode::BAD_REQUEST,
    StatusCode::UNAUTHORIZED,
    StatusCode::PAYMENT_REQUIRED,
    StatusCode::FORBIDDEN,
    StatusCode::NOT_FOUND,
    StatusCode::METHOD_NOT_ALLOWED,
    StatusCode::NOT_ACCEPTABLE,
    StatusCode::PROXY_AUTHENTICATION_REQUIRED,
    StatusCode::REQUEST_TIMEOUT,
    StatusCode::CONFLICT,
    StatusCode::GONE,
    StatusCode::LENGTH_REQUIRED,
    StatusCode::PRECONDITION_FAILED,
    StatusCode::PAYLOAD_TOO_LARGE,
    StatusCode::URI_TOO_LONG,
    StatusCode::UNSUPPORTED_MEDIA_TYPE,
    StatusCode::RANGE_NOT_SATISFIABLE,
    StatusCode::EXPECTATION_FAILED,
    StatusCode::IM_A_TEAPOT,
    StatusCode::MISDIRECTED_REQUEST,
    StatusCode::UNPROCESSABLE_ENTITY,
    StatusCode::LOCKED,
    StatusCode::FAILED_DEPENDENCY,
    StatusCode::UPGRADE_REQUIRED,
    StatusCode::PRECONDITION_REQUIRED,
    StatusCode::TOO_MANY_REQUESTS,
    StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
    StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
    // all 5XX response codes
    StatusCode::INTERNAL_SERVER_ERROR,
    StatusCode::NOT_IMPLEMENTED,
    StatusCode::BAD_GATEWAY,
    StatusCode::SERVICE_UNAVAILABLE,
    StatusCode::GATEWAY_TIMEOUT,
    StatusCode::HTTP_VERSION_NOT_SUPPORTED,
    StatusCode::VARIANT_ALSO_NEGOTIATES,
    StatusCode::INSUFFICIENT_STORAGE,
    StatusCode::LOOP_DETECTED,
    StatusCode::NOT_EXTENDED,
    StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
];
