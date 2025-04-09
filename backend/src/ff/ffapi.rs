use crate::core;
use crate::model;
use crate::model::ff::videoadaptationset::VideoAdaptationSet;
use crate::model::ff::videoadaptationset::VideoRepresentation;
use actix_web::dev::HttpServiceFactory;
use actix_web::get;
use actix_web::{post, web, Responder};
use utoipa_actix_web::OpenApiFactory;

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = bool),
    )
)]
#[post("/update")]
pub async fn update(
    data: web::Json<model::ff::InFFUpdate>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    core.ffdb
        .put_ff_config(data.config.clone())
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Fail to update config: {}", e))
        })?;
    Ok(web::Json(true))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = bool),
    )
)]
#[post("/set_config_state")]
pub async fn set_config_state(
    data: web::Json<model::InSetState>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    core.ffdb
        .set_ff_config_state(&data.id.server_uid, &data.id.config_uid, data.state)
        .await
        .ok();
    Ok(web::Json(true))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = bool),
    )
)]
#[post("/remove")]
pub async fn remove(
    data: web::Json<model::InConfigID>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    core.ffdb
        .remove_ff_config(&data.server_uid, &data.config_uid)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Fail to remove config: {}", e))
        })?;
    Ok(web::Json(true))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = model::ff::OutGetAllFFConfig),
    )
)]
#[get("/get_all_configs")]
pub async fn get_all_configs(core: web::Data<core::Core>) -> actix_web::Result<impl Responder> {
    let configs = core.ffdb.get_all_ff_configs().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Fail to get all configs: {}", e))
    })?;
    Ok(web::Json(model::ff::OutGetAllFFConfig { configs }))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = model::ff::OutGetFFStatus),
    )
)]
#[get("/get_all_status")]
pub async fn get_all_status(core: web::Data<core::Core>) -> actix_web::Result<impl Responder> {
    let status = core.ffdb.get_all_ff_status().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Fail to get all status: {}", e))
    })?;
    Ok(web::Json(model::ff::OutGetFFStatus { status }))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = model::ff::videoadaptationset::VideoRepresentation),
    )
)]
#[get("/create_default_video_representation")]
pub async fn create_default_video_representation() -> actix_web::Result<impl Responder> {
    Ok(web::Json(VideoRepresentation::new()))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = model::ff::videoadaptationset::VideoAdaptationSet),
    )
)]
#[get("/create_default_video_adaptation_set")]
pub async fn create_default_video_adaptation_set() -> actix_web::Result<impl Responder> {
    Ok(web::Json(VideoAdaptationSet::new()))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = String),
    )
)]
#[post("/get_ff_command")]
pub async fn get_ff_command(
    data: web::Json<model::InConfigID>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    let ret = core
        .get_ff_cmd(&data.server_uid, &data.config_uid)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Fail to get ff cmd: {}", e))
        })?;
    Ok(web::Json(ret))
}

#[utoipa::path(
    tags = ["FF"],
    responses(
        (status = 200, description = "OK", body = String),
    )
)]
#[post("/create")]
pub async fn create(
    data: web::Json<model::ff::InFFCreate>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    let uid = core
        .ffdb
        .create_ff_config(&data.server_uid, &data.name, &data.output)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Fail to create ff config: {}", e))
        })?;
    Ok(web::Json(uid))
}

pub fn service() -> impl HttpServiceFactory + OpenApiFactory + 'static {
    utoipa_actix_web::scope("/ff")
        .service(create_default_video_representation)
        .service(create_default_video_adaptation_set)
        .service(get_ff_command)
        .service(create)
        .service(update)
        .service(set_config_state)
        .service(remove)
        .service(get_all_configs)
        .service(get_all_status)
}
