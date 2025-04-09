use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq,ToSchema)]
#[repr(u8)]
pub enum CoreState {
    Stopped,
    Waiting,
    Running,
    Error,
}

impl CoreState {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => CoreState::Stopped,
            1 => CoreState::Waiting,
            2 => CoreState::Running,
            _ => CoreState::Error,
        }
    }
}
