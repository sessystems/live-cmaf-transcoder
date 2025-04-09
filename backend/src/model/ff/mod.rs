use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod ffconfig;
pub mod videoadaptationset;
pub mod audioadaptationset;
pub mod srt;
pub mod videofilter;
pub mod ffstatusvalue;


#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct InFFCreate {
    pub server_uid: String,
    pub name: String,
    pub output: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct OutGetFFStatus {
    pub status: Vec<ffstatusvalue::FFStatusValue>,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct InFFUpdate {
    pub config: ffconfig::FFConfig,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct OutGetAllFFConfig {
    pub configs: Vec<ffconfig::FFConfig>,
}
