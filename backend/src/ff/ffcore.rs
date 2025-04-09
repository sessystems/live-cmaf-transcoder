use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::model::server::Server;

use super::{ffdb, ffprocess};

#[derive(Clone)]
pub struct FFCore {
    pub cores: Arc<Mutex<HashMap<String, ffprocess::FFProcessHolder>>>,
    pub output: std::path::PathBuf,
    pub ffmpeg: String,
    pub modified_ffmpeg: bool,
    pub server_info: Server,
}

impl FFCore {
    pub fn new(
        output: &std::path::Path,
        ffmpeg: String,
        modified_ffmpeg: bool,
        server_info: Server,
    ) -> Self {
        Self {
            cores: Arc::new(Mutex::new(HashMap::new())),
            output: output.to_path_buf(),

            ffmpeg,
            modified_ffmpeg,
            server_info,
        }
    }

    pub async fn init_ff_cores(&self, ffdb: &ffdb::FFDb) -> std::io::Result<()> {
        let configs = ffdb.get_server_ff_configs(&self.server_info.uid).await?;
        for config in configs {
            let core = ffprocess::FFProcessHolder::new(
                self.server_info.clone(),
                config.uid.clone(),
                ffdb.clone(),
                self.output.clone(),
                self.ffmpeg.clone(),
                self.modified_ffmpeg,
            );
            core.get().refresh().await;
            self.cores.lock().unwrap().insert(config.uid.clone(), core);
        }
        Ok(())
    }

    pub async fn refresh_ff_core(
        &self,
        config_uid: &str,
        ffdb: &ffdb::FFDb,
    ) -> std::io::Result<()> {
        let core = self
            .cores
            .lock()
            .unwrap()
            .get(config_uid)
            .map(|core| core.get());

        if let Some(core) = core {
            core.refresh().await;
            return Ok(());
        }

        let new_core = ffprocess::FFProcessHolder::new(
            self.server_info.clone(),
            config_uid.to_owned(),
            ffdb.clone(),
            self.output.clone(),
            self.ffmpeg.clone(),
            self.modified_ffmpeg,
        );

        new_core.get().refresh().await;
        self.cores
            .lock()
            .unwrap()
            .insert(config_uid.to_owned(), new_core);
        Ok(())
    }

    pub async fn remove_ff_core(&self, config_uid: &str) {
        self.cores.lock().unwrap().remove(config_uid);
    }
}
