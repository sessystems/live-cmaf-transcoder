use redis::{aio::ConnectionManager, AsyncCommands};

use crate::{
    db::dbredis::DBRedis,
    model::{self, ff::ffconfig, state},
    new_io_error, utils,
};

#[derive(Clone)]
pub struct FFDb {
    pub redis: DBRedis,
}

impl FFDb {
    pub fn new(redis: DBRedis) -> Self {
        Self { redis }
    }

    pub fn con_url(&self) -> &str {
        &self.redis.conn_url
    }

    pub async fn set_ff_status(
        &mut self,
        status: model::ff::ffstatusvalue::FFStatusValue,
        expired_seconds: i64,
    ) -> std::io::Result<()> {
        let key = format!("ff:status:{}", status.config_uid);
        let value = serde_json::to_string(&status).unwrap();
        DBRedis::set_expires(&key, &value, &mut self.redis.con.clone(), expired_seconds).await
    }

    pub async fn get_all_ff_status(
        &self,
    ) -> std::io::Result<Vec<model::ff::ffstatusvalue::FFStatusValue>> {
        let keys = "ff:status:*";
        let mut con = self.redis.con.clone();
        DBRedis::get_all(keys, &mut con).await
    }

    fn config_id(server_uid: &str, uid: &str) -> String {
        format!("ff:config:{}:{}", server_uid, uid)
    }

    pub async fn get_server_ff_configs(
        &self,
        server_uid: &str,
    ) -> std::io::Result<Vec<model::ff::ffconfig::FFConfig>> {
        let mut con = self.redis.con.clone();
        let keys = format!("ff:config:{}:*", server_uid);
        DBRedis::get_all(&keys, &mut con).await
    }

    pub async fn get_all_ff_configs(&self) -> std::io::Result<Vec<model::ff::ffconfig::FFConfig>> {
        let mut con = self.redis.con.clone();
        DBRedis::get_all("ff:config:*", &mut con).await
    }

    pub async fn remove_ff_config(&self, server_uid: &str, uid: &str) -> std::io::Result<()> {
        let config_id = Self::config_id(server_uid, uid);
        self.redis.con.clone().del(&config_id).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to remove ff config {:?}", e)
            )
        })
    }

    pub async fn create_ff_config(
        &self,
        server_uid: &str,
        name: &str,
        path: &str,
    ) -> std::io::Result<String> {
        let path = utils::to_path_checked(path)?;
        let server_info = self.redis.get_server(server_uid).await?;
        let config_uid = uuid::Uuid::new_v4().to_string();
        let config = model::ff::ffconfig::FFConfig::new(
            &server_info,
            config_uid.clone(),
            name.to_owned(),
            path.to_path_buf(),
        );
        self.put_ff_config(config).await?;
        Ok(config_uid)
    }

    pub async fn put_ff_config(
        &self,
        config: model::ff::ffconfig::FFConfig,
    ) -> std::io::Result<()> {
        let mut con = self.redis.con.clone();
        Self::set_ff_config(config, &mut con).await
    }

    async fn set_ff_config(
        config: model::ff::ffconfig::FFConfig,
        con: &mut ConnectionManager,
    ) -> std::io::Result<()> {
        let config_id = Self::config_id(&config.server_uid, &config.uid);
        DBRedis::set_object(&config_id, &config, con).await
    }

    async fn get_ff_config_internal(
        server_uid: &str,
        uid: &str,
        con: &mut ConnectionManager,
    ) -> std::io::Result<ffconfig::FFConfig> {
        let config_id = Self::config_id(server_uid, uid);
        DBRedis::get(&config_id, con).await
    }

    pub async fn set_ff_config_state(
        &self,
        server_uid: &str,
        uid: &str,
        state: state::State,
    ) -> std::io::Result<bool> {
        let mut con = self.redis.con.clone();
        let mut config = Self::get_ff_config_internal(server_uid, uid, &mut con).await?;
        if config.state == state {
            return Ok(false);
        }

        config.state = state;
        Self::set_ff_config(config, &mut self.redis.con.clone()).await?;
        Ok(true)
    }

    pub async fn get_ff_config(
        &self,
        server_uid: &str,
        uid: &str,
    ) -> std::io::Result<ffconfig::FFConfig> {
        let mut con = self.redis.con.clone();
        Self::get_ff_config_internal(server_uid, uid, &mut con).await
    }

    pub async fn get_ff_config_mut(
        &mut self,
        server_uid: &str,
        uid: &str,
    ) -> std::io::Result<ffconfig::FFConfig> {
        Self::get_ff_config_internal(server_uid, uid, &mut self.redis.con).await
    }
}
