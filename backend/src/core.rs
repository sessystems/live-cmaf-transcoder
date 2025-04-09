/*---------------------------------------------------------------------------------------------
 *  Copyright 2024 SES
 *  Licensed under the Apache 2.0 License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use crate::ff::{ffcommand, ffdb};
use crate::{db::dbredis, model::server::Server};

use crate::{ff, new_io_error};

#[derive(Clone)]
pub struct Core {
    pub redis: dbredis::DBRedis,
    pub ff_core: Option<ff::ffcore::FFCore>,
    pub ffdb: ffdb::FFDb,
    server_info: Server,
}

impl Core {
    pub fn new(
        ffoutput: &std::path::Path,
        redis: dbredis::DBRedis,
        ffmpeg: Option<String>,
        modified_ffmpeg: bool,
        server_info: Server,
    ) -> Self {
        Self {
            ff_core: ffmpeg.map(|ffmpeg| {
                ff::ffcore::FFCore::new(ffoutput, ffmpeg, modified_ffmpeg, server_info.clone())
            }),
            redis: redis.clone(),
            ffdb: ffdb::FFDb::new(redis.clone()),
            server_info,
        }
    }

    pub async fn init(&self) -> std::io::Result<()> {
        if let Some(ff_core) = self.ff_core.as_ref() {
            ff_core.init_ff_cores(&self.ffdb).await?;
        }

        Ok(())
    }

    pub async fn get_ff_cmd(&self, server_uid: &str, uid: &str) -> std::io::Result<String> {
        let config = self.ffdb.get_ff_config(server_uid, uid).await?;
        let gpu = self.server_info.get_gpu(&config.gpu_uid);
        if gpu.is_none() {
            return Err(new_io_error!(std::io::ErrorKind::NotFound, "GPU not found"));
        }
        let cmd = ffcommand::FFCommand::new(
            gpu.as_ref().unwrap(),
            &config,
            std::path::Path::new("/"),
            false,
        );
        Ok(cmd.to_string())
    }
}
