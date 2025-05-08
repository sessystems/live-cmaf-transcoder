// Copyright 2024 SES
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Authors: Yannick Poirier

use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::model::{
    corestate::CoreState,
    server::{self, ServerCapability},
};
use actix_files::Files;
use actix_web::{
    http,
    middleware::{self, Logger},
    web::{self},
    App, HttpServer,
};
use clap::Parser;
use ff::ffapi;

use ff::ffmpegbin::FFmpegBinList;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::{Config, SwaggerUi};

mod api;
mod core;
mod db;
mod ff;
mod frontend;
mod model;
mod servermonitoring;
mod utils;

#[macro_export]
macro_rules! new_io_error {
    ($kind:expr, $msg:expr) => {{
        let msg = $msg;
        tracing::error!("{:?}", msg);
        std::io::Error::new($kind, $msg)
    }};
}

/// CMAF Encoder
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Web server binding address
    /// ex: 0.0.0.0:8888
    #[arg(long)]
    bind_addr: Option<String>,

    /// Path to FFMPEG binary built with GPL codecs
    #[arg(long, default_value = "ffmpeg")]
    ffmpeg: Vec<String>,

    /// Path to FFMPEG binary built with non-free codecs
    #[arg(long)]
    ffmpeg_non_free: Option<String>,

    /// Directory to store live output files (segments and manifests)
    #[arg(long)]
    live_output: Option<std::path::PathBuf>,

    /// The unique identifier (UID) of this server.
    /// This can also be set through the `SERVER_UID` environment variable.
    /// It is used to uniquely identify the server instance.
    /// If not provided, the system's native machine ID will be used as a fallback.
    #[arg(long)]
    uid: Option<String>,

    /// The name of this server.
    /// This can also be set through the `SERVER_NAME` environment variable.
    /// It is used to display the server name in the web interface.
    /// If not provided, the system's hostname will be used as a default.
    #[arg(long)]
    name: Option<String>,

    /// Public base URL of this server, ex: http://localhost:8888
    /// This can also be set through the `BASE_URL` environment variable.
    #[arg(long)]
    base_url: Option<String>,

    /// Redis URI redis(s)://username:password@host:port
    /// This can also be set through the `REDIS_URL` environment variable.
    /// If not provided, a default Redis URI redis://:${REDIS_PASSWORD}@localhost:6379 be used.
    #[arg(long)]
    redis: Option<String>,

    /// Disable Transcoder Module
    #[arg(long)]
    disable_transcoder: Option<bool>,

    /// Disable the User Interface
    #[arg(long)]
    disable_ui: Option<bool>,
}

fn get_env(key: &str) -> Option<String> {
    let env = env::var(key).ok()?;
    match env.is_empty() {
        true => None,
        false => Some(env),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_line_number(true)
        .init();
    let args = Args::parse();

    let bind_addr = args
        .bind_addr
        .clone()
        .or_else(|| get_env("BIND_ADDR"))
        .unwrap_or_else(|| "0.0.0.0:8888".to_string());

    let port: u16 = bind_addr
        .split(':')
        .last()
        .unwrap_or("80")
        .parse()
        .unwrap_or(80);

    let base_url = args
        .base_url
        .clone()
        .or_else(|| get_env("BASE_URL"))
        .unwrap_or_else(|| format!("http://{}:{}", local_ip_address::local_ip().unwrap(), port));

    url::Url::parse(&base_url)
        .unwrap_or_else(|_| panic!("baseURL {} is not a valid URL", base_url));

    let server_uid = args
        .uid
        .clone()
        .or_else(|| get_env("SERVER_UID"))
        .unwrap_or_else(|| {
            let id = machine_uid::get().expect("Server UID not found");
            let mut s = DefaultHasher::new();
            id.hash(&mut s);
            s.finish().to_string()
        });

    let server_name = args
        .name
        .clone()
        .or_else(|| get_env("SERVER_NAME"))
        .unwrap_or_else(|| {
            hostname::get()
                .expect("Fail to get hostname")
                .into_string()
                .expect("Fail to convert hostname to string")
        });

    tracing::info!("Base URL: {}", base_url);
    tracing::info!("Server UID: {}", server_uid);
    tracing::info!("Server Name: {}", server_name);

    let disable_transcoder = args
        .disable_transcoder
        .or_else(|| match get_env("DISABLE_TRANSCODER") {
            Some(value) => value.parse::<bool>().ok(),
            None => None,
        })
        .unwrap_or(false);

    let mut ffmpegs: Option<FFmpegBinList> = None;
    if !disable_transcoder {
        let ret = FFmpegBinList::new(&args.ffmpeg)
            .await
            .expect("FFMpeg binary is not valid");
        ffmpegs = Some(ret);
    }

    let redis_url = args
        .redis
        .clone()
        .or_else(|| get_env("REDIS_URL"))
        .or_else(|| {
            let password = get_env("REDIS_PASSWORD")?;
            Some(format!("redis://:{}@localhost:6379", password))
        })
        .unwrap_or_else(|| "redis://localhost:6379".to_string());

    let mut redis = match crate::db::dbredis::DBRedis::new(&redis_url).await {
        Ok(redis) => redis,
        Err(e) => {
            tracing::error!("Fail to connect to redis database: {}", e);
            return Err(std::io::Error::from(std::io::ErrorKind::ConnectionRefused));
        }
    };

    if let Ok(server_status) = redis.get_server_status(&server_uid).await {
        if server_status.current_state == CoreState::Running {
            tracing::error!("Server with the same UID is already running...");
            return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists));
        }
    }

    let mut service_capability = Vec::new();

    let disable_ui = args
        .disable_ui
        .or_else(|| match get_env("DISABLE_UI") {
            Some(value) => value.parse::<bool>().ok(),
            None => None,
        })
        .unwrap_or(false);

    if !disable_transcoder {
        service_capability.push(ServerCapability::Transcode)
    }

    if !disable_ui {
        service_capability.push(ServerCapability::UserInterface)
    }

    let server_info = server::Server::new(
        server_uid.clone(),
        server_name,
        base_url,
        port,
        service_capability,
    )
    .await;

    let live_output_dir = args.live_output.clone().unwrap_or_else(|| {
        let temp_path: std::path::PathBuf = env::temp_dir().join("live-cmaf-transcoder-output");
        match temp_path.exists() {
            true => temp_path,
            false => match std::fs::create_dir_all(&temp_path) {
                Ok(_) => temp_path,
                Err(_) => {
                    tracing::error!("Fail to create live output directory {:?}", temp_path);
                    temp_path
                }
            },
        }
    });

    if !live_output_dir.exists() {
        tracing::error!("Fail to find live output directory : {:?}", live_output_dir);
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }

    tracing::info!("Live Output Directory: {:?}", live_output_dir);

    let core = core::Core::new(
        &live_output_dir,
        redis.clone(),
        ffmpegs,
        server_info.clone(),
    );

    redis.set_server(server_info).await.ok();
    core.init().await.expect("Fail to init state");

    let mut ffevent = core.ff_core.as_ref().map(|ff_core| {
        ff::ffevent::FFEvent::new(ff_core.clone(), &core.ffdb).expect("Fail to create FFEvent")
    });

    let mut server_monitoring =
        servermonitoring::ServerMonitoring::new(server_uid.clone(), redis.clone());
    let server_monitoring = tokio::spawn(async move {
        server_monitoring.run().await;
    });
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = env!("CARGO_PKG_NAME"),
            contact(name = "Yannick Poirier <yannick.poirier@ses.com>"),
            description = "API for the Live CMAF Transcoder",
            license(name = "Apache 2.0", identifier = "Apache-2.0"),
            version = env!("CARGO_PKG_VERSION")
        ),
    )]
    struct ApiDoc;

    tracing::info!("Start Web Server: {}", bind_addr);
    HttpServer::new(move || {
        let cmaf_service = Files::new("/", live_output_dir.clone())
            .disable_content_disposition()
            .prefer_utf8(true);

        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_header(http::header::CONTENT_TYPE);

        let mut app = App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(core.clone()))
            .wrap(actix_web::middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-cache")))
            .wrap(cors)
            .into_utoipa_app()
            .openapi(ApiDoc::openapi());

        if !disable_ui {
            app = app.service(
                utoipa_actix_web::scope("/api")
                    .service(api::get_all_servers)
                    .service(api::get_all_server_status)
                    .service(api::remove_server)
                    .service(api::get_logs)
                    .service(ffapi::service()),
            );
        }

        let (mut app, openapi) = app.split_for_parts();
        if !disable_ui {
            app = app
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .config(Config::default())
                        .url("/api-docs/openapi.json", openapi.clone()),
                )
                .service(frontend::index)
                .service(frontend::embedded_file)
        }

        app.service(cmaf_service)
    })
    .bind(&bind_addr)
    .unwrap()
    .run()
    .await
    .expect("Failed to run Actix web server");

    server_monitoring.abort();

    if let Some(ffevent) = ffevent.as_mut() {
        ffevent.stop().await;
    }

    redis.remove_server_status(&server_uid).await.ok();

    Ok(())
}
