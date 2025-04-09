use std::hash::{DefaultHasher, Hash, Hasher};

use redis::{aio::ConnectionManager, AsyncCommands};
use serde::Serialize;

use crate::{
    model::{self, server},
    new_io_error,
};

#[derive(Clone)]
pub struct DBRedis {
    pub conn_url: String,
    pub con: ConnectionManager,
}

impl DBRedis {
    pub async fn new(redis_conn_url: &str) -> std::io::Result<Self> {
        tracing::info!("Connecting to redis at {}", redis_conn_url);
        let client = redis::Client::open(redis_conn_url).map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to open redis {}", e)
            )
        })?;

        let con = ConnectionManager::new(client).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to open redis {}", e)
            )
        })?;

        Ok(DBRedis {
            conn_url: redis_conn_url.to_string(),
            con,
        })
    }

    pub async fn set_server(&self, server: server::Server) -> std::io::Result<()> {
        let key: String = format!("server:{}", server.uid);
        Self::set_object(&key, &server, &mut self.con.clone()).await
    }

    pub async fn get_server(&self, server_uid: &str) -> std::io::Result<server::Server> {
        let key = format!("server:{}", server_uid);
        Self::get(&key, &mut self.con.clone()).await
    }

    pub async fn get_all_servers(&self) -> std::io::Result<Vec<server::Server>> {
        let keys = "server:*";
        let mut con = self.con.clone();
        let keys: Vec<String> = con.keys(keys).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get keys {:?}", e)
            )
        })?;
        let values = Self::get_values(&keys, &mut con).await?;
        let values: Vec<server::Server> = values
            .iter()
            .filter_map(|value| serde_json::from_str(value).ok())
            .collect();

        Ok(values)
    }

    pub async fn get_server_status(
        &self,
        server_uid: &str,
    ) -> std::io::Result<model::serverstatus::ServerStatus> {
        let key = format!("server_status:{}", server_uid);
        Self::get(&key, &mut self.con.clone()).await
    }

    pub async fn remove_server(&self, server_uid: &str) -> std::io::Result<()> {
        let configs_key = format!("config:{}:*", server_uid);
        let server_key = format!("server:{}", server_uid);
        let server_status_key = format!("server_status:{}", server_uid);
        let mut con = self.con.clone();

        if let Ok(server_status) =
            Self::get::<model::serverstatus::ServerStatus>(&server_status_key, &mut con).await
        {
            if server_status.current_state == model::corestate::CoreState::Running {
                return Err(new_io_error!(
                    std::io::ErrorKind::Other,
                    "Running Servers cannot be deleted"
                ));
            }
        }

        let configs: Vec<String> = con.keys(&configs_key).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get keys {:?}", e)
            )
        })?;

        let _: Option<()> = con.del(configs).await.ok();
        let _: Option<()> = con.del(&server_key).await.ok();
        Ok(())
    }

    pub async fn set_server_status(
        &mut self,
        status: model::serverstatus::ServerStatus,
        expired_seconds: i64,
    ) -> std::io::Result<()> {
        let key = format!("server_status:{}", status.server_uid);
        let value = serde_json::to_string(&status).unwrap();
        Self::set_expires(&key, &value, &mut self.con.clone(), expired_seconds).await
    }

    pub async fn remove_server_status(&mut self, server_uid: &str) -> std::io::Result<()> {
        let key = format!("server_status:{}", server_uid);
        self.con.del(key).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to remove server status {:?}", e)
            )
        })
    }

    pub async fn get_all_server_status(
        &self,
    ) -> std::io::Result<Vec<model::serverstatus::ServerStatus>> {
        let keys = "server_status:*";
        let mut con = self.con.clone();
        let keys: Vec<String> = con.keys(keys).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get keys {:?}", e)
            )
        })?;
        let values = Self::get_values(&keys, &mut con).await?;
        let values: Vec<model::serverstatus::ServerStatus> = values
            .iter()
            .filter_map(|value| serde_json::from_str(value).ok())
            .collect();

        Ok(values)
    }

    pub async fn set_object<T>(
        key: &str,
        obj: &T,
        con: &mut ConnectionManager,
    ) -> std::io::Result<()>
    where
        T: ?Sized + Serialize,
    {
        let jobj = serde_json::to_string(&obj)?;
        con.clone().set(key, jobj).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to set obj {:?}", e)
            )
        })
    }

    pub async fn get<T>(key: &str, con: &mut ConnectionManager) -> std::io::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let data: Vec<u8> = con.get(key).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get obj {:?}", e)
            )
        })?;

        let ret = serde_json::from_slice(&data)?;
        Ok(ret)
    }

    pub async fn get_all<T>(
        key_pattern: &str,
        con: &mut ConnectionManager,
    ) -> std::io::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let keys: Vec<String> = con.keys(key_pattern).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get keys {:?}", e)
            )
        })?;

        let values = DBRedis::get_values(&keys, con).await?;
        let values: Vec<T> = values
            .iter()
            .filter_map(|value| serde_json::from_str(value).ok())
            .collect();

        Ok(values)
    }

    pub async fn set_expires(
        key: &str,
        value: &str,
        con: &mut ConnectionManager,
        expired_seconds: i64,
    ) -> std::io::Result<()> {
        let _: () = redis::pipe()
            .atomic()
            .set(key, value)
            .ignore()
            .expire(key, expired_seconds)
            .ignore()
            .query_async(con)
            .await
            .map_err(|e| {
                new_io_error!(std::io::ErrorKind::Other, format!("Fail to set {:?}", e))
            })?;
        Ok(())
    }

    pub async fn get_values(
        keys: &[String],
        con: &mut ConnectionManager,
    ) -> std::io::Result<Vec<String>> {
        if keys.is_empty() {
            return Ok(vec![]);
        }

        con.get(keys)
            .await
            .map_err(|_| new_io_error!(std::io::ErrorKind::Other, "Fail to get values from redis"))
    }

    pub async fn add_log(
        &mut self,
        config_uid: &str,
        log: model::logs::Log,
        expired_seconds: i64,
    ) -> std::io::Result<()> {
        let mut hasher = DefaultHasher::new();
        log.hash(&mut hasher);
        let key = format!("logs:{}:{:x}", config_uid, hasher.finish());
        let value = serde_json::to_string(&log).unwrap();
        Self::set_expires(&key, &value, &mut self.con, expired_seconds).await
    }

    pub async fn get_logs(&self, config_uid: &str) -> std::io::Result<Vec<model::logs::Log>> {
        let keys = format!("logs:{}:*", config_uid);
        let mut con = self.con.clone();
        let keys: Vec<String> = con.keys(keys).await.map_err(|e| {
            new_io_error!(
                std::io::ErrorKind::Other,
                format!("Fail to get keys {:?}", e)
            )
        })?;
        let values = Self::get_values(&keys, &mut con).await?;
        let mut values: Vec<model::logs::Log> = values
            .iter()
            .filter_map(|value| serde_json::from_str(value).ok())
            .collect();

        values.sort_by_cached_key(|log| std::cmp::Reverse(log.timestamp));
        Ok(values)
    }
}
