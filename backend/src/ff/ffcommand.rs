use crate::model::{
    ff::{
        ffconfig::{self, Acceleration},
        videoadaptationset::VideoAdaptationSet,
    },
    server::Gpu,
};

#[derive(Debug)]
pub struct FFCommand {
    output: std::path::PathBuf,
    args: Vec<String>,
}

impl std::fmt::Display for FFCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: Vec<String> = self
            .args()
            .iter()
            .map(|s| match s.starts_with('-') {
                true => s.to_string(),
                false => format!("\"{}\"", s.replace('$', "\\$")),
            })
            .collect();

        write!(f, "ffmpeg {}", args.join(" "))
    }
}

impl FFCommand {
    pub fn new(
        gpu: &Gpu,
        config: &ffconfig::FFConfig,
        base_output: &std::path::Path,
        modified: bool,
    ) -> Self {
        let output = base_output.join(config.output.strip_prefix("/").unwrap_or(&config.output));
        let output_dash = output.join("manifest.mpd");
        let metadata = format!("title={}", config.name);
        let segment_duration = config.segment_duration_ms as f64 / 1000.0;

        let mut args = Vec::new();
        args.push("-progress".to_string());
        args.push("/dev/stdout".to_string());
        args.push("-stats_period".to_string());
        args.push("2.0".to_string());
        args.push("-loglevel".to_string());
        args.push("repeat+level+info".to_string());
        args.push("-hide_banner".to_string());
        args.push("-stats".to_string());
        args.push("-fflags".to_string());
        args.push("+genpts".to_string());

        if gpu.acceleration == Acceleration::Vaapi {
            args.push("-init_hw_device".to_string());
            args.push(format!("vaapi=vaapi0:{}", gpu.uid));
            args.push("-hwaccel".to_string());
            args.push("vaapi".to_string());
            args.push("-hwaccel_output_format".to_string());
            args.push("vaapi".to_string());
            args.push("-hwaccel_device".to_string());
            args.push("vaapi0".to_string());
            args.push("-filter_hw_device".to_string());
            args.push("vaapi0".to_string());
            args.push("-extra_hw_frames".to_string());
            args.push("3".to_string());
        } else if gpu.acceleration == Acceleration::Cuda {
            args.push("-hwaccel".to_string());
            args.push("cuda".to_string());
            args.push("-hwaccel_output_format".to_string());
            args.push("cuda".to_string());
            args.push("-hwaccel_device".to_string());
            args.push(gpu.index.to_string());
            args.push("-extra_hw_frames".to_string());
            args.push("3".to_string());
        }

        args.push("-err_detect".to_string());
        args.push("ignore_err".to_string());

        let input = config.input.to_string();
        let input_url = url::Url::parse(&input);
        if let Ok(input_url) = input_url {
            if input_url.scheme() == "file"
                || input_url.scheme() == "http"
                || input_url.scheme() == "https"
            {
                args.push("-re".to_string());
            }
        } else {
            args.push("-re".to_string());
            args.push("-stream_loop".to_string());
            args.push("-1".to_string())
        }

        args.push("-i".to_string());
        args.push(input);

        args.push("-metadata".to_string());
        args.push(metadata);

        args.push("-flags".to_string());
        args.push("+global_header ".to_string());

        args.push("-c:a".to_string());
        args.push(config.audio_adaptation_set.encoder.to_string());
        args.push("-ar".to_string());
        args.push(config.audio_adaptation_set.sample_rate.to_string());
        args.push("-b:a".to_string());
        args.push(config.audio_adaptation_set.bitrate.to_string());
        args.push("-metadata:s:a:0".to_string());
        args.push(format!("role={}", config.audio_adaptation_set.role));

        args.push("-profile:a".to_string());
        args.push(config.audio_adaptation_set.profile.to_string());

        if !config.video_adaptation_set.is_empty() {
            args.push("-color_primaries".to_string());
            args.push(config.colorspace.to_string());
            args.push("-color_trc".to_string());
            args.push(config.colorspace.to_string());
            args.push("-colorspace".to_string());
            args.push(config.colorspace.to_string());

            args.push("-filter_complex".to_string());
            let mut filter_complex = String::new();

            if config.deinterlace.enable {
                match gpu.acceleration {
                    Acceleration::Vaapi => {
                        filter_complex.push_str("deinterlace_vaapi=mode=default:rate=field,");
                    }
                    Acceleration::Cuda => {
                        filter_complex.push_str("bwdif_cuda=mode=send_field:parity=auto,");
                    }
                    _ => {
                        filter_complex.push_str("bwdif=mode=1:parity=-1:deint=0,");
                    }
                }
            }

            filter_complex.push_str(&VideoAdaptationSet::ff_split_filter(
                gpu.acceleration,
                &config.video_adaptation_set,
            ));
            args.push(filter_complex);
        }

        args.push("-noautoscale".to_string());

        VideoAdaptationSet::ff_video_encode(gpu, config, &config.video_adaptation_set, &mut args);
        VideoAdaptationSet::ff_map(&config.video_adaptation_set, &mut args);

        args.push("-map".to_string());
        args.push("0:a:0".to_string());

        args.push("-adaptation_sets".to_string());

        let vas =
            VideoAdaptationSet::ff_adaptation_sets(segment_duration, &config.video_adaptation_set);

        args.push(format!(
            "{vas}id={},seg_duration={segment_duration},streams=a",
            config.video_adaptation_set.len()
        ));

        args.push("-window_size".to_string());
        args.push(config.window_size.to_string());

        args.push("-remove_at_exit".to_string());
        args.push("1".to_string());

        args.push("-use_timeline".to_string());
        args.push(match config.mpd_type {
            ffconfig::MPDType::SegmentTimeline => "1".to_string(),
            ffconfig::MPDType::Template => "0".to_string(),
        });

        args.push("-use_template".to_string());
        args.push("1".to_string());

        args.push("-hls_master_name".to_string());
        args.push("playlist.m3u8".to_string());

        if config.enable_hls {
            args.push("-hls_playlist".to_string());
            args.push("1".to_string());
        }

        args.push("-update_period".to_string());
        args.push(segment_duration.to_string());

        args.push("-streaming".to_string());
        args.push("1".to_string());

        args.push("-media_seg_name".to_string());
        args.push(config.media_seg_name.to_string());

        args.push("-init_seg_name".to_string());
        args.push(config.init_seg_name.to_string());

        args.push("-index_correction".to_string());
        args.push("1".to_string());

        if let Some(utc_timing_url) = config.utc_timing_url.as_ref() {
            args.push("-utc_timing_url".to_string());
            args.push(utc_timing_url.to_string());
        }

        if modified {
            args.push("-ast_delay_us".to_string());
            args.push(format!(
                "{}",
                (config.segment_duration_ms + config.ast_delay_ms) * 1000
            ));
        }

        args.push("-format_options".to_string());
        let mut format_options = "movflags=cmaf".to_string();
        if let (Some(key), Some(kid)) = (
            config.encryption_key.as_ref(),
            config.encryption_kid.as_ref(),
        ) {
            format_options.push_str(&format!(
                ":encryption_scheme=cenc-aes-ctr:encryption_key={}:encryption_kid={}",
                key, kid
            ));
        }
        args.push(format_options);

        args.push("-mpd_profile".to_string());
        args.push("dash".to_string());

        args.push("-f".to_string());
        args.push("dash".to_string());
        args.push(output_dash.to_str().unwrap().to_string());

        Self { output, args }
    }

    pub fn output_folder(&self) -> &std::path::Path {
        &self.output
    }

    pub fn args(&self) -> Vec<&str> {
        self.args.iter().map(|s| s.as_str()).collect()
    }
}
