
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use super::corestate::CoreState;

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct ServerStatus {
    pub server_uid: String,
    pub cpu_usage: u32,
    pub nb_cpus: u32,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub current_state: CoreState,
}