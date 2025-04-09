use serde::{Deserialize, Serialize};
use utoipa::{ToSchema,IntoParams};

use self::logs::Log;

pub mod corestate;
pub mod ff;
pub mod logs;
pub mod server;
pub mod serverstatus;
pub mod state;

#[derive(Clone, Deserialize, Serialize, Debug,ToSchema,IntoParams)]
pub struct InConfigID {
    pub server_uid: String,
    pub config_uid: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema, IntoParams)]
pub struct InServerID {
    pub server_uid: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct OutGetAllServers {
    pub servers: Vec<server::Server>,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct OutGetAllServerStatus {
    pub status: Vec<serverstatus::ServerStatus>,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct InSetState {
    pub id: InConfigID,
    pub state: state::State,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct OutGetLogs {
    pub logs: Vec<Log>,
}
