use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::server::Gpu;

use super::{
    ffconfig::{Acceleration, FFConfig},
    videofilter::VideoFilterDrawText,
};

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum Colorspace {
    BT709,
}

impl Display for Colorspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colorspace::BT709 => write!(f, "bt709"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum VideoEncoder {
    H264,
    HEVC,
}

impl VideoEncoder {
    pub fn to_string(&self, acceleration: Acceleration) -> &str {
        match self {
            VideoEncoder::H264 => match acceleration {
                Acceleration::Software => "libx264",
                Acceleration::Vaapi => "h264_vaapi",
                Acceleration::Cuda => "h264_nvenc",
            },
            VideoEncoder::HEVC => match acceleration {
                Acceleration::Software => "libx265",
                Acceleration::Vaapi => "hevc_vaapi",
                Acceleration::Cuda => "hevc_nvenc",
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum Preset {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
}

impl Preset {
    pub fn _vaapi_compression_level(&self) -> u8 {
        match self {
            Preset::Ultrafast => 1,
            Preset::Superfast => 1,
            Preset::Veryfast => 2,
            Preset::Faster => 3,
            Preset::Fast => 4,
            Preset::Medium => 5,
            Preset::Slow => 6,
            Preset::Slower => 7,
            Preset::Veryslow => 7,
        }
    }
}

impl Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Preset::Ultrafast => write!(f, "ultrafast"),
            Preset::Superfast => write!(f, "superfast"),
            Preset::Veryfast => write!(f, "veryfast"),
            Preset::Faster => write!(f, "faster"),
            Preset::Fast => write!(f, "fast"),
            Preset::Medium => write!(f, "medium"),
            Preset::Slow => write!(f, "slow"),
            Preset::Slower => write!(f, "slower"),
            Preset::Veryslow => write!(f, "veryslow"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub enum Profile {
    Main,
    High,
    High10,
    High422,
    High444,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Profile::Main => write!(f, "main"),
            Profile::High => write!(f, "high"),
            Profile::High10 => write!(f, "high10"),
            Profile::High422 => write!(f, "high422"),
            Profile::High444 => write!(f, "high444"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct VideoAdaptationSet {
    pub encoder: VideoEncoder,
    pub representations: Vec<VideoRepresentation>,
    pub framerate_num: u32,
    pub framerate_den: u32,
    pub aspect_ratio_num: Option<u32>,
    pub aspect_ratio_den: Option<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct VideoRepresentation {
    pub width: u32,
    pub height: u32,
    pub bitrate: u32,
    pub max_bitrate: u32,
    pub buf_size: Option<u32>,
    pub preset: Preset,
    pub profile: Profile,
    pub text: VideoFilterDrawText,
}

impl VideoRepresentation {
    pub fn new() -> Self {
        Self {
            width: 1920,
            height: 1080,
            bitrate: 8000000,
            max_bitrate: 8000000,
            buf_size: None,
            preset: Preset::Medium,
            profile: Profile::Main,
            text: VideoFilterDrawText::new(),
        }
    }
}

impl VideoRepresentation {
    pub fn ff_sw_filter(&self, acceleration: Acceleration) -> String {
        let mut filter = String::new();
        if self.text.enabled {
            if acceleration == Acceleration::Vaapi || acceleration == Acceleration::Cuda {
                filter.push_str("hwdownload,format=nv12,");
            }
            filter.push_str(&self.text.ff_drawtext());
            if acceleration == Acceleration::Vaapi {
                filter.push_str(",hwupload,format=vaapi");
            } else if acceleration == Acceleration::Cuda {
                filter.push_str(",hwupload_cuda");
            }
        }

        filter
    }
}

impl VideoAdaptationSet {
    pub fn new() -> Self {
        Self {
            encoder: VideoEncoder::H264,
            representations: vec![VideoRepresentation::new()],
            framerate_num: 50,
            framerate_den: 1,
            aspect_ratio_num: None,
            aspect_ratio_den: None,
        }
    }

    pub fn gop_size(&self, segment_duration_ms: u64) -> u64 {
        (self.framerate_num as u64 * segment_duration_ms)
            .checked_div(1000 * self.framerate_den as u64)
            .unwrap_or(1)
    }

    pub fn ff_adaptation_sets(segment_duration: f64, sets: &[VideoAdaptationSet]) -> String {
        let mut index = 0;
        let mut adaptation_sets_id = 0;
        let mut adaptation_sets = String::new();
        for adaptation in sets.iter() {
            if !adaptation.representations.is_empty() {
                adaptation_sets.push_str(&format!("id={},", adaptation_sets_id));
                adaptation_sets.push_str(&format!("seg_duration={},", segment_duration));
                adaptation_sets.push_str("streams=");
                for _ in 0..adaptation.representations.len() {
                    adaptation_sets.push_str(&format!("{index},"));
                    index += 1;
                }

                adaptation_sets.pop();
                adaptation_sets.push(' ');
                adaptation_sets_id += 1;
            }
        }
        adaptation_sets
    }

    pub fn ff_video_encode(
        gpu: &Gpu,
        config: &FFConfig,
        sets: &[VideoAdaptationSet],
        args: &mut Vec<String>,
    ) {
        let mut index = 0;
        for adaptation in sets.iter() {
            for representation in adaptation.representations.iter() {
                if gpu.acceleration == Acceleration::Software {
                    args.push(format!("-sc_threshold:v:{index}"));
                    args.push("0".to_string());
                }

                args.push(format!("-g:v:{index}"));
                args.push(format!(
                    "{}",
                    adaptation.gop_size(config.segment_duration_ms as u64)
                ));
                args.push(format!("-keyint_min:v:{index}"));
                args.push(format!(
                    "{}",
                    adaptation.gop_size(config.segment_duration_ms as u64)
                ));

                args.push(format!("-r:{index}"));
                args.push(format!(
                    "{}/{}",
                    adaptation.framerate_num, adaptation.framerate_den
                ));
                args.push(format!("-c:v:{index}"));
                args.push(adaptation.encoder.to_string(gpu.acceleration).to_owned());

                if gpu.acceleration == Acceleration::Cuda {
                    args.push("-gpu".to_string());
                    args.push(gpu.index.to_string());
                }

                args.push(format!("-b:v:{index}"));
                args.push(representation.bitrate.to_string());
                args.push(format!("-maxrate:v:{index}"));
                args.push(representation.max_bitrate.to_string());
                args.push(format!("-bufsize:v:{index}"));
                args.push(
                    representation
                        .buf_size
                        .unwrap_or(representation.max_bitrate / 2)
                        .to_string(),
                );
                args.push(format!("-profile:v:{index}"));
                args.push(representation.profile.to_string());

                if gpu.acceleration == Acceleration::Software {
                    args.push(format!("-preset:v:{index}"));
                    args.push(representation.preset.to_string());
                }

                index += 1;
            }
        }
    }

    pub fn ff_map(sets: &[VideoAdaptationSet], args: &mut Vec<String>) {
        for (adaptation_i, adaptation) in sets.iter().enumerate() {
            for (representation_i, _representation) in adaptation.representations.iter().enumerate()
            {
                let stream_id = format!("[s{adaptation_i}_{representation_i}]");
                args.push("-map".to_string());
                args.push(stream_id);
            }
        }
    }

    pub fn ff_split_filter(acceleration: Acceleration, sets: &[VideoAdaptationSet]) -> String {
        let mut filter = String::new();
        let nb: usize = sets.iter().map(|x| x.representations.len()).sum();

        filter.push_str(&format!("split={nb}",));
        for (adaptation_i, adaptation) in sets.iter().enumerate() {
            for (representation_i, _) in adaptation.representations.iter().enumerate() {
                filter.push_str(&format!("[s{adaptation_i}_{representation_i}]"));
            }
        }
        filter.push(';');
        for (adaptation_i, adaptation) in sets.iter().enumerate() {
            for (representation_i, representation) in adaptation.representations.iter().enumerate()
            {
                if representation.width > 0 && representation.height > 0
                    || representation.text.enabled
                {
                    let stream_id = format!("[s{adaptation_i}_{representation_i}]");
                    filter.push_str(&stream_id);
                    filter.push_str(&representation.ff_sw_filter(acceleration));

                    if representation.width > 0 && representation.height > 0 {
                        if representation.text.enabled {
                            filter.push(',');
                        }

                        match acceleration {
                            Acceleration::Vaapi => {
                                filter.push_str(&format!(
                                    "scale_vaapi=w={}:h={}",
                                    representation.width, representation.height
                                ));
                            }
                            Acceleration::Cuda => {
                                filter.push_str(&format!(
                                    "scale_cuda=w={}:h={}",
                                    representation.width, representation.height
                                ));
                            }
                            Acceleration::Software => {
                                filter.push_str(&format!(
                                    "scale=w={}:h={}",
                                    representation.width, representation.height,
                                ));
                            }
                        }

                        if let (Some(num), Some(den)) =
                            (adaptation.aspect_ratio_num, adaptation.aspect_ratio_den)
                        {
                            filter.push_str(&format!(",setdar={}/{}", num, den));
                        }
                    }
                    filter.push_str(&stream_id);
                    filter.push(';');
                }
            }
        }

        filter.pop();
        filter
    }
}
