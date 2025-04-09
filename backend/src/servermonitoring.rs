/*---------------------------------------------------------------------------------------------
 *  Copyright 2024 SES
 *  Licensed under the Apache 2.0 License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
 
use crate::{
    db::dbredis::DBRedis,
    model::{corestate::CoreState, serverstatus::ServerStatus},
};

pub struct ServerMonitoring {
    server_uid: String,
    db: DBRedis,
}

impl ServerMonitoring {
    pub fn new(server_uid: String, db: DBRedis) -> Self {
        Self { server_uid, db }
    }

    pub async fn run(&mut self) {
        let mut s = sysinfo::System::new();
        loop {
            s.refresh_cpu_all();
            s.refresh_memory();

            let cpu_usage: f32 = s.cpus().iter().map(|c| c.cpu_usage()).sum();

            let status = ServerStatus {
                server_uid: self.server_uid.clone(),
                cpu_usage: (cpu_usage * 100.0) as u32,
                nb_cpus: s.cpus().len() as u32,
                memory_usage: s.used_memory(),
                total_memory: s.total_memory(),
                current_state: CoreState::Running,
            };

            self.db.set_server_status(status, 5).await.ok();
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
}
