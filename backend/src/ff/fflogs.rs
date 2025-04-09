use crate::model::logs::{self, Log};

use super::ffdb::FFDb;

#[derive(Clone)]
pub struct FFLogs {
    config_uid: String,
    ffdb: FFDb,
    warning_expiration_time_s: i64,
    error_expiration_time_s: i64,
    default_expiration_time_s: i64,
}

impl FFLogs {
    pub fn new(
        ffdb: FFDb,
        config_uid: String,
        default_expiration_time_s: i64,
        warning_expiration_time_s: i64,
        error_expiration_time_s: i64,
    ) -> Self {
        Self {
            ffdb,
            config_uid,
            warning_expiration_time_s,
            error_expiration_time_s,
            default_expiration_time_s,
        }
    }

    pub async fn push(&mut self, fftext: &str) {
        let log = Log::new(fftext);
        let expiration = match log.level {
            logs::LogLevel::Error => self.error_expiration_time_s,
            logs::LogLevel::Warning => self.warning_expiration_time_s,
            _ => self.default_expiration_time_s,
        };

        self.ffdb
            .redis
            .add_log(&self.config_uid, log, expiration)
            .await
            .ok();
    }
}
