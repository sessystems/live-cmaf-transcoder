use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Hash, ToSchema)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Deserialize, Serialize, Debug, Hash, PartialEq, Eq, ToSchema)]
pub struct Log {
    pub timestamp: u64,
    pub text: String,
    pub level: LogLevel,
}

impl Log {
    pub fn new(fftext: &str) -> Self {
        let level = match fftext {
            s if s.contains("[panic]") => LogLevel::Error,
            s if s.contains("[fatal]") => LogLevel::Error,
            s if s.contains("[error]") => LogLevel::Error,
            s if s.contains("[warning]") => LogLevel::Warning,
            _ => LogLevel::Info,
        };

        let end_of_level = fftext.rfind("]").unwrap_or(0);
        let text = fftext[end_of_level + 1..].trim();
        if level != LogLevel::Info {
            tracing::error!("{:?} {:?}", level, text);
        }

        Self {
            timestamp: chrono::Utc::now().timestamp() as u64,
            text: text.to_owned(),
            level,
        }
    }
}
