use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    sync::{Arc, RwLock},
    time::Duration,
};

use actix_web::rt::{spawn, time::interval};
use chrono::{DateTime, Utc};
use log::debug;
use reqwest::{ClientBuilder, StatusCode};
use uuid::Uuid;

use crate::models::service_info::service_info_entity::ServiceInfoEntity;

struct HealthCheckTask {
    service: Arc<RwLock<ServiceInfoEntity>>,
    next_healthcheck: DateTime<Utc>,
}

impl HealthCheckTask {
    fn find_next_health_check_time(service: &ServiceInfoEntity) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::seconds(service.interval.unwrap_or(30).into())
    }

    async fn health_check_service(&mut self) -> Result<(), ()> {
        let local_service = &mut self.service.write().unwrap(); //Clone value and get lock again after await or drop value and re get the lock
        let complete_url = "http://".to_owned()
            + &local_service.host
            + ":"
            + &local_service.port.to_string()
            + &local_service.health_check_endpoint;

        debug!(
            "Healthchecking service {:?} at {:?}",
            local_service.service_name.to_owned() + "-" + &local_service.id.to_owned(),
            complete_url
        );

        let client = ClientBuilder::new()
            .timeout(Duration::new(local_service.timeout.unwrap_or(30).into(), 0))
            .build()
            .unwrap();

        match client
            .execute(client.get(complete_url).build().unwrap())
            .await
        {
            Err(e) => {
                local_service.health_check_fails += 1;
                if local_service.health_check_fails > 3 {
                    Err(())
                } else {
                    self.next_healthcheck =
                        HealthCheckTask::find_next_health_check_time(local_service);
                    Ok(())
                }
            }
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    local_service.health_check_fails = 0;
                    self.next_healthcheck =
                        HealthCheckTask::find_next_health_check_time(local_service);
                    Ok(())
                }
                _ => {
                    local_service.health_check_fails += 1;
                    if local_service.health_check_fails > 3 {
                        Err(())
                    } else {
                        self.next_healthcheck =
                            HealthCheckTask::find_next_health_check_time(local_service);
                        Ok(())
                    }
                }
            },
        }
    }
}

impl Eq for HealthCheckTask {}

impl PartialEq for HealthCheckTask {
    fn eq(&self, other: &Self) -> bool {
        self.service
            .read()
            .unwrap()
            .eq(&other.service.read().unwrap())
            && self.next_healthcheck.eq(&other.next_healthcheck)
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
    registered_services: Arc<RwLock<HashMap<String, Vec<Arc<RwLock<ServiceInfoEntity>>>>>>,
    health_check_queue: Arc<RwLock<BinaryHeap<HealthCheckTask>>>,
}

impl RegistrationService {
    pub fn new() -> Self {
        Self {
            registered_services: Arc::new(RwLock::new(HashMap::new())),
            health_check_queue: Arc::new(RwLock::new(BinaryHeap::new())),
        }
    }

    pub fn run(&self) {
        let internal_queue_pointer = self.health_check_queue.clone();
        spawn(async move {
            let mut interval = interval(Duration::from_millis(1));
            interval.tick().await;
            loop {
                interval.tick().await;
                let mut map = internal_queue_pointer.write().unwrap();
                if map.peek() == None {
                    continue;
                }
                while let Some(mut task) = map.pop() {
                    if Utc::now()
                        .signed_duration_since(task.next_healthcheck)
                        .num_seconds()
                        >= 0
                    {
                        spawn(async move {
                            match task.health_check_service().await {
                                Err(_) => {
                                    todo!()
                                }
                                Ok(_) => {
                                    todo!()
                                }
                            }
                        });
                    }
                }
            }
        });
    }

    pub fn register_service(&mut self, mut service: ServiceInfoEntity) -> ServiceInfoEntity {
        service.id = Uuid::new_v4().to_string();

        let rc_service = Arc::new(RwLock::new(service.clone()));

        let readable_service = rc_service.read().unwrap();

        match self
            .registered_services
            .write()
            .unwrap()
            .entry(readable_service.service_name.clone())
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
            next_healthcheck: HealthCheckTask::find_next_health_check_time(&readable_service),
        };

        self.health_check_queue
            .write()
            .unwrap()
            .push(health_check_task);

        service
    }

    pub fn server_exists_by_id(self, id: &String) -> Option<ServiceInfoEntity> {
        match self.registered_services.read().unwrap().get(id) {
            Some(vector_of_services) => {
                let mut vector_of_services_iter = vector_of_services.iter();
                match vector_of_services_iter
                    .find(|element| -> bool { element.read().unwrap().id.eq(id) })
                {
                    Some(service) => {
                        let readable_service = service.read().unwrap();
                        let new_value = ServiceInfoEntity::new(
                            readable_service.id.clone(),
                            readable_service.service_name.clone(),
                            readable_service.host.clone(),
                            readable_service.port,
                            readable_service.health_check_endpoint.clone(),
                            readable_service.interval,
                            readable_service.timeout,
                            readable_service.health_check_fails,
                        );
                        Some(new_value)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn deregister_service(&self, service: &ServiceInfoEntity) {
        match self
            .registered_services
            .read()
            .unwrap()
            .get(&service.service_name)
        {
            Some(vector) => {
                let mut vector_of_services_iter = vector.iter();
                match vector_of_services_iter
                    .find(|element| -> bool { element.read().unwrap().id.eq(&service.id) })
                {
                    Some(service) => {
                        todo!()
                    }
                    None => {
                        todo!()
                    }
                }
            }
            None => {}
        }
    }
}
