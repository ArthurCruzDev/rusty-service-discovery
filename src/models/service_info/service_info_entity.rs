use super::service_info_register_dto::ServiceInfoRegisterDto;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ServiceInfoEntity {
    pub id: String,
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub health_check_endpoint: String,
    pub interval: Option<u32>,
    pub timeout: Option<u32>,
}

impl ServiceInfoEntity {
    pub fn new(
        id: String,
        service_name: String,
        host: String,
        port: u16,
        health_check_endpoint: String,
        interval: Option<u32>,
        timeout: Option<u32>,
    ) -> ServiceInfoEntity {
        ServiceInfoEntity {
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

impl TryFrom<ServiceInfoRegisterDto> for ServiceInfoEntity {
    type Error = &'static str;

    fn try_from(dto: ServiceInfoRegisterDto) -> Result<Self, Self::Error> {
        if dto.host.is_none() {
            Err("Cannot create ServiceInfo entity without a host!")
        } else {
            Ok(Self {
                id: dto.id,
                service_name: dto.service_name.trim().to_lowercase(),
                host: dto.host.unwrap(),
                port: dto.port,
                health_check_endpoint: dto.health_check_endpoint,
                interval: dto.interval,
                timeout: dto.timeout,
            })
        }
    }
}

impl PartialEq for ServiceInfoEntity {
    fn eq(&self, other: &Self) -> bool {
        self.service_name.eq_ignore_ascii_case(&other.service_name)
            && self.host.eq_ignore_ascii_case(&other.host)
            && self.port == other.port
    }
}
