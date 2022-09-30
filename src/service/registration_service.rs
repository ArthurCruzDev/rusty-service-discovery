use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    error::Error,
    rc::{self, Rc},
};

use actix_web::dev::Service;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::models::service_info::service_info_entity::ServiceInfoEntity;

struct HealthCheckTask {
    service: Rc<ServiceInfoEntity>,
    next_healthcheck: DateTime<Utc>,
}

impl Eq for HealthCheckTask {}

impl PartialEq for HealthCheckTask {
    fn eq(&self, other: &Self) -> bool {
        self.service.eq(&other.service) && self.next_healthcheck.eq(&other.next_healthcheck)
    }
}

impl PartialOrd for HealthCheckTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HealthCheckTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.next_healthcheck.cmp(&self.next_healthcheck)
    }
}

#[derive(Default)]
pub struct RegistrationService {
    registered_services: HashMap<String, Vec<Rc<ServiceInfoEntity>>>,
    health_check_queue: BinaryHeap<HealthCheckTask>,
}

impl RegistrationService {
    pub fn new() -> Self {
        Self {
            registered_services: HashMap::new(),
            health_check_queue: BinaryHeap::new(),
        }
    }

    pub fn register_service(&mut self, mut service: ServiceInfoEntity) -> String {
        service.id = Uuid::new_v4().to_string();

        let rc_service = Rc::new(service);

        match self
            .registered_services
            .entry(rc_service.service_name.clone())
        {
            Entry::Vacant(e) => {
                e.insert(vec![rc_service.clone()]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(rc_service.clone());
            }
        }

        let health_check_task = HealthCheckTask {
            service: rc_service.clone(),
            next_healthcheck: Utc::now()
                + Duration::seconds(rc_service.interval.unwrap_or(30).into()),
        };

        self.health_check_queue.push(health_check_task);

        rc_service.id.clone()
    }

    pub fn server_exists_by_id(&self, id: &String) -> Option<&ServiceInfoEntity> {
        match self.registered_services.get(id) {
            Some(vector_of_services) => {
                let mut vector_of_services_iter = vector_of_services.iter();
                match vector_of_services_iter.find(|element| -> bool { element.id.eq(id) }) {
                    Some(service) => Some(service),
                    None => None,
                }
            }
            None => None,
        }
    }
}
