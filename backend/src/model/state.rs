use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, Debug, Copy, PartialEq, Eq, ToSchema)]
pub enum State {
    Active,
    Inactive,
}
