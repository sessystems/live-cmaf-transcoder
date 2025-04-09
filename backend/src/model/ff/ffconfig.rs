use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::{server::Server, state::State};

use super::{
    audioadaptationset::AudioAdaptationSet,
    srt::Srt,
    videoadaptationset::{Colorspace, VideoAdaptationSet},
    videofilter::VideoFilterDeinterlace,
};

#[derive(Default, Clone, Deserialize, Serialize, Debug, Copy, PartialEq, Eq, ToSchema)]
pub enum Acceleration {
    Software,
    #[default]
    Vaapi,
    Cuda,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct FFConfig {
    pub uid: String,
    pub server_uid: String,
    pub name: String,
    pub state: State,
    pub input: Srt,
    pub output: String,
    pub segment_duration_ms: u32,
    pub colorspace: Colorspace,
    pub deinterlace: VideoFilterDeinterlace,
    pub video_adaptation_set: Vec<VideoAdaptationSet>,
    pub audio_adaptation_set: AudioAdaptationSet,
    pub mpd_type: MPDType,
    pub enable_hls: bool,
    pub media_seg_name: String,
    pub init_seg_name: String,
    pub utc_timing_url: Option<String>,
    pub ast_delay_ms: u32,
    pub window_size: u32,
    #[serde(default)]
    pub acceleration: Acceleration,
    pub gpu_uid: String,
    pub encryption_key: Option<String>,
    pub encryption_kid: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy, PartialEq, Eq, ToSchema)]
pub enum MPDType {
    Template,
    SegmentTimeline,
}

impl FFConfig {
    pub fn new(
        server_info: &Server,
        config_uid: String,
        name: String,
        output: std::path::PathBuf,
    ) -> Self {
        let gpu = server_info.get_default_gpu();
        Self {
            uid: config_uid,
            name,
            server_uid: server_info.uid.clone(),
            state: State::Inactive,
            input: Srt::new(),
            output: output.to_string_lossy().into_owned(),
            segment_duration_ms: 2000,
            colorspace: Colorspace::BT709,
            deinterlace: VideoFilterDeinterlace::new(),
            video_adaptation_set: vec![VideoAdaptationSet::new()],
            audio_adaptation_set: AudioAdaptationSet::new(),
            mpd_type: MPDType::Template,
            enable_hls: true,
            media_seg_name: "$RepresentationID$-$Number%05d$.$ext$".to_string(),
            init_seg_name: "init_$RepresentationID$.$ext$".to_string(),
            utc_timing_url: Some("http://time.akamai.com?iso&amp;ms".to_string()),
            ast_delay_ms: 200,
            window_size: 10,
            acceleration: gpu.acceleration,
            gpu_uid: gpu.uid.clone(),
            encryption_key: None,
            encryption_kid: None,
        }
    }
}
