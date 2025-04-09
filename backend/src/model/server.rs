use serde::{Deserialize, Serialize};

use tokio::fs;
use utoipa::ToSchema;

use super::ff::ffconfig::Acceleration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct Gpu {
    pub acceleration: Acceleration,
    pub index: u32,
    pub uid: String,
    pub name: String,
}

impl Default for Gpu {
    fn default() -> Self {
        Gpu {
            acceleration: Acceleration::Software,
            index: 0,
            uid: "sw".to_string(),
            name: "Software".to_string(),
        }
    }
}

impl Gpu {
    async fn new_vec() -> Vec<Self> {
        let mut ret = Vec::new();
        ret.push(Gpu::default());
        Self::push_vaapi(&mut ret).await;
        Self::push_nvidia(&mut ret).await;
        tracing::info!("GPUs: {:?}", ret);
        ret
    }

    async fn push_nvidia(result: &mut Vec<Self>) {
        let process = tokio::process::Command::new("nvidia-smi")
            .arg("--query-gpu=index,name,uuid")
            .arg("--format=csv,noheader")
            .stdout(std::process::Stdio::piped())
            .output()
            .await;

        if let Ok(process) = process {
            let output = String::from_utf8_lossy(&process.stdout).to_string();
            for line in output.lines() {
                let mut parts = line.split(", ");
                let index = parts.next().map(|p| p.parse::<u32>().unwrap_or_default());
                let name = parts.next().map(|p| p.to_string());
                let uid = parts.next().map(|p| p.to_string());
                if index.is_none() || name.is_none() || uid.is_none() {
                    continue;
                }
                result.push(Gpu {
                    acceleration: Acceleration::Cuda,
                    index: index.unwrap(),
                    uid: uid.unwrap(),
                    name: name.unwrap(),
                });
            }
        }
    }

    async fn push_vaapi(result: &mut Vec<Self>) {
        let files = fs::read_dir("/dev/dri").await;
        let mut index = 0;
        if let Ok(mut files) = files {
            while let Ok(Some(file)) = files.next_entry().await {
                let filename = file.file_name().to_string_lossy().to_string();
                if !filename.starts_with("renderD") {
                    continue;
                }

                let process: Result<std::process::Output, std::io::Error> =
                    tokio::process::Command::new("vainfo")
                        .arg("--display")
                        .arg("drm")
                        .arg("--device")
                        .arg(format!("/dev/dri/{}", filename))
                        .stdout(std::process::Stdio::piped())
                        .output()
                        .await;

                let mut name = String::new();
                let mut enc = false;
                if let Ok(process) = process {
                    let output = String::from_utf8_lossy(&process.stdout).to_string();
                    for line in output.lines() {
                        if line.starts_with("vainfo: Driver version: ") {
                            name = line["vainfo: Driver version:".len()..].to_string();
                        }
                        if !enc && line.contains("VAEntrypointEnc") {
                            enc = true;
                        }
                    }
                }

                if enc {
                    result.push(Gpu {
                        acceleration: Acceleration::Vaapi,
                        index,
                        uid: format!("/dev/dri/{}", filename),
                        name,
                    });
                    index += 1;
                }
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Copy, ToSchema)]
pub enum ServerCapability {
    Transcode,
    DVBI,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct Server {
    pub uid: String,
    pub name: String,
    pub base_url: String,
    pub port: u16,
    pub version: String,
    pub gpus: Vec<Gpu>,
    pub capabilities: Vec<ServerCapability>,
}

impl Server {
    pub async fn new(
        uid: String,
        name: String,
        base_url: String,
        port: u16,
        capabilities: Vec<ServerCapability>,
    ) -> Self {
        Server {
            uid,
            name,
            base_url,
            port,
            version: VERSION.to_string(),
            gpus: Gpu::new_vec().await,
            capabilities,
        }
    }

    pub fn get_gpu(&self, gpu_uid: &str) -> Option<&Gpu> {
        self.gpus.iter().find(|gpu| gpu.uid == gpu_uid)
    }

    pub fn get_default_gpu(&self) -> Gpu {
        let default_gpu = Gpu::default();
        self.gpus
            .iter()
            .find(|gpu| gpu.acceleration == Acceleration::Vaapi)
            .unwrap_or_else(|| {
                self.gpus
                    .iter()
                    .find(|gpu| gpu.acceleration == Acceleration::Cuda)
                    .unwrap_or_else(|| {
                        self.gpus
                            .iter()
                            .find(|gpu| gpu.acceleration == Acceleration::Software)
                            .unwrap_or(&default_gpu)
                    })
            })
            .clone()
    }
}
