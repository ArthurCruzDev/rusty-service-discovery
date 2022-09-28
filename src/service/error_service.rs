use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{self, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use crate::models::error::error_entity::ErrorEntity;

#[derive(Debug, Deserialize)]
struct ErrorFile {
    version: String,
    error_mappings: HashMap<u16, String>,
}

#[derive(Debug, Clone)]
pub struct ErrorService {
    pub version: String,
    error_mapping: HashMap<u16, String>,
}

impl ErrorService {
    pub fn new() -> Self {
        let read_file_result = File::open("errors.json");

        if let Ok(error_file) = read_file_result {
            let reader = BufReader::new(error_file);

            let reader_result: Result<ErrorFile> = serde_json::from_reader(reader);
            match reader_result {
                Ok(read_error_file) => ErrorService {
                    version: read_error_file.version,
                    error_mapping: read_error_file.error_mappings,
                },
                Err(e) => {
                    panic!(
                        "The application was unable to parse the errors file to an internal structure: {:?}", e
                    )
                }
            }
        } else {
            panic!("The application was unable to open the errors file")
        }
    }

    pub fn get_error_text(&self, error_code: &u16) -> Option<&String> {
        self.error_mapping.get(error_code)
    }

    pub fn get_error_entity_from_code(&self, error_code: &u16) -> Option<ErrorEntity> {
        self.error_mapping
            .get(error_code)
            .map(|error_text| ErrorEntity::new(*error_code, error_text.to_owned(), Utc::now()))
    }
}
