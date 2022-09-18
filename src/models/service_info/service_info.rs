use serde::Serialize;

use super::service_info_register_dto::ServiceInfoRegisterDto;

#[derive(Serialize, Debug)]
pub struct ServiceInfo {
    id: String,
    service_name: String,
    host: String,
    port: u16,
    health_check_endpoint: String,
    interval: Option<u32>,
    timeout: Option<u32>,
}

impl ServiceInfo {
    pub fn new(
        id: String,
        service_name: String,
        host: String,
        port: u16,
        health_check_endpoint: String,
        interval: Option<u32>,
        timeout: Option<u32>,
    ) -> ServiceInfo {
        ServiceInfo {
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

impl TryFrom<ServiceInfoRegisterDto> for ServiceInfo {
    type Error = &'static str;

    fn try_from(dto: ServiceInfoRegisterDto) -> Result<Self, Self::Error> {
        if dto.host.is_none() {
            Err("Cannot create ServiceInfo entity without a host!")
        } else {
            Ok(Self {
                id: dto.id,
                service_name: dto.service_name,
                host: dto.host.unwrap(),
                port: dto.port,
                health_check_endpoint: dto.health_check_endpoint,
                interval: dto.interval,
                timeout: dto.timeout,
            })
        }
    }
}
