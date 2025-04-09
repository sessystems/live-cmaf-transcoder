use crate::db::subscribermanager::{self, EventHandler, SubscriberManager};
use async_trait::async_trait;

use super::{ffcore::FFCore, ffdb::FFDb};

pub struct FFEvent {
    mgr: SubscriberManager,
}

struct FFEventHandler {
    ffcore: FFCore,
    ffdb: FFDb,
}

impl FFEvent {
    pub fn new(ffcore: FFCore, ffdb: &FFDb) -> std::io::Result<Self> {
        let channels = vec![format!(
            "__keyspace@0__:ff:config:{}:*",
            &ffcore.server_info.uid
        )];

        let redis_con = ffdb.con_url().to_owned();
        let handler = Box::new(FFEventHandler::new(ffcore, ffdb.clone()));
        let mgr = SubscriberManager::new(&redis_con, handler, channels)?;
        Ok(FFEvent { mgr })
    }

    pub async fn stop(&mut self) {
        self.mgr.stop().await;
    }
}

impl FFEventHandler {
    pub fn new(ffcore: FFCore, ffdb: FFDb) -> Self {
        FFEventHandler { ffcore, ffdb }
    }

    async fn handle_config(&mut self, evt: &subscribermanager::Event) {
        let config_uid = evt.channel.split(':').last();
        if config_uid.is_none() {
            tracing::error!("Invalid event channel: {}", evt.channel);
            return;
        }

        if evt.payload == "set" {
            tracing::info!("Refresh cores");
            self.ffcore
                .refresh_ff_core(config_uid.unwrap(), &self.ffdb)
                .await
                .ok();
        } else if evt.payload == "del" {
            tracing::info!("Delete cores");
            self.ffcore.remove_ff_core(config_uid.unwrap()).await;
        } else {
            tracing::info!("Invalid event payload: {}", evt.payload);
        }
    }
}

#[async_trait]
impl EventHandler for FFEventHandler {
    async fn on_event(&mut self, evt: &subscribermanager::Event) {
        self.handle_config(evt).await;
    }
}
