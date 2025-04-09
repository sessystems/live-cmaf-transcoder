/*---------------------------------------------------------------------------------------------
 *  Copyright 2024 SES
 *  Licensed under the Apache 2.0 License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use crate::model::{self};
use actix_web::{get, post, web, Responder};

use crate::core;

#[utoipa::path(
    tags = ["Servers"],
    responses(
        (status = 200, description = "OK", body = model::OutGetAllServers),
    )
)]
#[get("/get_all_servers")]
pub async fn get_all_servers(core: web::Data<core::Core>) -> actix_web::Result<impl Responder> {
    let servers = core.redis.get_all_servers().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Fail to get all servers: {}", e))
    })?;
    Ok(web::Json(model::OutGetAllServers { servers }))
}

#[utoipa::path(
    tags = ["Servers"],
    responses(
        (status = 200, description = "OK", body = model::OutGetAllServerStatus),
    )
)]
#[get("/get_all_server_status")]
pub async fn get_all_server_status(
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    let status = core.redis.get_all_server_status().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Fail to get all server status: {}", e))
    })?;
    Ok(web::Json(model::OutGetAllServerStatus { status }))
}

#[utoipa::path(
    tags = ["Servers"],
    
    responses(
        (status = 200, description = "OK", body = model::OutGetLogs),
    )
)]
#[post("/get_logs")]
pub async fn get_logs(
    data: web::Json<model::InConfigID>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    let logs = core.redis.get_logs(&data.config_uid).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Fail to get all logs: {}", e))
    })?;
    Ok(web::Json(model::OutGetLogs { logs }))
}

#[utoipa::path(
    tags = ["Servers"],
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/remove_server")]
pub async fn remove_server(
    data: web::Json<model::InServerID>,
    core: web::Data<core::Core>,
) -> actix_web::Result<impl Responder> {
    let ret: () = core
        .redis
        .remove_server(&data.server_uid)
        .await  // This is a blocking operation, we have to use `await` here
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Fail to get remove the server: {}",
                e
            ))
        })?;
    Ok(web::Json(ret))
}
