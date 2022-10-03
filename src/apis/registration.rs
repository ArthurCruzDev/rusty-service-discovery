use crate::{
    models::{
        error::error_dto::ErrorDto,
        service_info::{
            service_info_entity::ServiceInfoEntity,
            service_info_register_dto::ServiceInfoRegisterDto,
        },
    },
    AppState,
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use log::error;
use std::convert::TryFrom;

#[post("/registration/register")]
pub async fn register(req: HttpRequest, body: Json<ServiceInfoRegisterDto>) -> HttpResponse {
    let mut dto = body.into_inner();
    if let Some(host_address) = req.connection_info().realip_remote_addr() {
        dto.host.replace(host_address.to_string());
    } else {
        let error_service = &req.app_data::<AppState>().unwrap().error_service;
        let error_entity = error_service.get_error_entity_from_code(&10000).unwrap();
        error!("{}", error_entity.msg);
        return HttpResponse::InternalServerError().json(ErrorDto::from(error_entity));
    }

    match ServiceInfoEntity::try_from(dto) {
        Ok(entity) => {
            let registered_service = &req
                .app_data::<AppState>()
                .unwrap()
                .registration_service
                .write()
                .unwrap()
                .register_service(entity);
            HttpResponse::Created().json(registered_service)
        }
        Err(_) => {
            let error_service = &req.app_data::<AppState>().unwrap().error_service;
            let error_entity = error_service.get_error_entity_from_code(&10001).unwrap();
            error!("{}", error_entity.msg);
            HttpResponse::InternalServerError().json(ErrorDto::from(error_entity))
        }
    }
}
