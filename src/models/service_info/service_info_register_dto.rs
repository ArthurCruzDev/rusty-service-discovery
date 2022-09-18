use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceInfoRegisterDto {
    pub id: String,
    pub service_name: String,
    pub host: Option<String>,
    pub port: u16,
    pub health_check_endpoint: String,
    pub interval: Option<u32>,
    pub timeout: Option<u32>,
}

impl ServiceInfoRegisterDto {
    pub fn new(
        id: String,
        service_name: String,
        host: Option<String>,
        port: u16,
        health_check_endpoint: String,
        interval: Option<u32>,
        timeout: Option<u32>,
    ) -> ServiceInfoRegisterDto {
        ServiceInfoRegisterDto {
            id,
            service_name,
            host,
            port,
            health_check_endpoint,
            interval,
            timeout,
        }
    }
}
