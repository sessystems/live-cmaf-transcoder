use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, Debug, Copy, PartialEq, Eq, ToSchema)]
pub enum SRTMode {
    Caller,
    Listener,
    Rendezvous,
}

impl Display for SRTMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SRTMode::Caller => write!(f, "caller"),
            SRTMode::Listener => write!(f, "listener"),
            SRTMode::Rendezvous => write!(f, "rendezvous"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct Srt {
    pub uri: String,
    pub mode: SRTMode,
    pub connect_timeout_ms: u64,
    pub srt_latency_us: i32,
}

impl Srt {
    pub fn new() -> Self {
        Self {
            uri: String::new(),
            mode: SRTMode::Listener,
            connect_timeout_ms: 5000,
            srt_latency_us: 200000,
        }
    }
}

impl Display for Srt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(mut url) = url::Url::parse(&self.uri) {
            if url.scheme() == "srt" {
                url.query_pairs_mut()
                    .append_pair("mode", &self.mode.to_string())
                    .append_pair(
                        "connect_timeout",
                        self.connect_timeout_ms.to_string().as_str(),
                    )
                    .append_pair("latency", &self.srt_latency_us.to_string().as_str());

                return write!(f, "{}", url.as_str());
            }
        }

        write!(f, "{}", self.uri)
    }
}
