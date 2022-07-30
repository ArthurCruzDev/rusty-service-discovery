use actix_web::{web, App, HttpResponse, HttpServer};
use log::info;
use std::env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let port_number = env::var("RUSTYSE_PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u16>()
        .expect("Failed to parse server port from enviroment or from default \"80\"");

    info!(
        "Starting Rusty Service Discovery Server on port: {}",
        port_number
    );

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", port_number))?
        .run()
        .await
}
