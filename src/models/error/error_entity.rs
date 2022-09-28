use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorEntity {
    pub error_code: u16,
    pub msg: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorEntity {
    pub fn new(error_code: u16, msg: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            error_code,
            msg,
            timestamp,
        }
    }
}
