use std::error::Error;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::error_entity::ErrorEntity;

#[derive(Debug, Serialize)]
pub struct ErrorDto {
    pub error_code: u16,
    pub msg: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorDto {
    pub fn new(error_code: u16, msg: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            error_code,
            msg,
            timestamp,
        }
    }
}

impl From<ErrorEntity> for ErrorDto {
    fn from(entity: ErrorEntity) -> Self {
        Self {
            error_code: entity.error_code,
            msg: entity.msg.to_owned(),
            timestamp: entity.timestamp,
        }
    }
}
