use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error, info, trace, warn};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let port_number = 80;

    info!(
        "Starting Rusty Service Discovery Server on port: {}",
        port_number
    );

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", port_number))?
        .run()
        .await
}
