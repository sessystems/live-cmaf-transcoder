use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum AudioEncoder {
    Aac,
    FDKAac,
}

impl Display for AudioEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioEncoder::Aac => write!(f, "aac"),
            AudioEncoder::FDKAac => write!(f, "libfdk_aac"),
        }
    }
}

#[derive(Default, Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum Audioprofile {
    #[default]
    Low,
    HEAAC,
    HEAACV2,
}

impl Display for Audioprofile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Audioprofile::Low => write!(f, "aac_low"),
            Audioprofile::HEAAC => write!(f, "aac_he"),
            Audioprofile::HEAACV2 => write!(f, "aac_he_v2"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum Role {
    Main,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Main => write!(f, "main"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct AudioAdaptationSet {
    pub encoder: AudioEncoder,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub role: Role,
    #[serde(default)]
    pub profile: Audioprofile,
}

impl AudioAdaptationSet {
    pub fn new() -> Self {
        Self {
            encoder: AudioEncoder::FDKAac,
            bitrate: 128000,
            sample_rate: 48000,
            role: Role::Main,
            profile: Audioprofile::Low,
        }
    }
}
