mod apis;
mod models;
mod service;

use actix_web::{
    rt::spawn,
    web::{self},
    App, HttpResponse, HttpServer,
};
use apis::registration::register;
use log::info;
use service::registration_service::RegistrationService;
use std::{
    env,
    sync::{Arc, RwLock},
};

use crate::service::error_service::ErrorService;

pub struct AppState {
    pub error_service: Arc<ErrorService>,
    pub registration_service: Arc<RwLock<RegistrationService>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let error_service = ErrorService::new();
    let registration_service = RegistrationService::new();

    let registration_service_arc_rwlock = Arc::new(RwLock::new(registration_service));

    registration_service_arc_rwlock
        .clone()
        .read()
        .unwrap()
        .run();

    let port_number = env::var("RUSTYSE_PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{}", error_service.get_error_text(&10002).unwrap()));

    info!(
        "Starting Rusty Service Discovery Server on port: {}",
        port_number
    );

    let error_service_arc = Arc::new(error_service);

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                error_service: error_service_arc.clone(),
                registration_service: registration_service_arc_rwlock.clone(),
            })
            .route("/", web::get().to(HttpResponse::Ok))
            .service(register)
    })
    .bind(("127.0.0.1", port_number))?
    .run()
    .await
}
