use std::convert::{TryFrom, TryInto};

use crate::models::service_info::{
    service_info::ServiceInfo, service_info_register_dto::ServiceInfoRegisterDto,
};
use actix_web::{
    get, post,
    web::{Json, Path},
    Error, HttpRequest, HttpResponse, Result,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[post("/registration/register")]
pub async fn register(req: HttpRequest, body: Json<ServiceInfoRegisterDto>) -> HttpResponse {
    let mut dto = body.into_inner();
    if let Some(host_address) = req.connection_info().realip_remote_addr() {
        dto.host.replace(host_address.to_string());
    } else {
        return HttpResponse::InternalServerError()
            .json("The server couldn't get the requesting service IP address!");
    }

    match ServiceInfo::try_from(dto) {
        Ok(entity) => HttpResponse::Created().json(entity),
        Err(_) => HttpResponse::InternalServerError()
            .json("The server was unable to correctly convert request data into internal data!"),
    }
}
