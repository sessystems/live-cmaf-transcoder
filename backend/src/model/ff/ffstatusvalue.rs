use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::corestate::CoreState;

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct FFStatusValue {
    pub config_uid: String,
    pub speed: f64,
    pub drop_frames: u64,
    pub duplicate_frames: u64,
    pub out_time_ms: u64,
    pub fps: f64,
    pub current_state: CoreState,
    pub cpu_usage: u32,
    pub memory_usage: u64,
    pub nb_restart: u32,
}
