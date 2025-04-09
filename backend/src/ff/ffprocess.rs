use std::sync::Arc;

use crate::model::server::Server;
use crate::model::{self, state};
use crate::new_io_error;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate};
use tokio::io::AsyncBufReadExt;
use tokio::process::{ChildStderr, ChildStdout, Command};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::ffcommand::FFCommand;
use super::ffdb::FFDb;
use super::fflogs::FFLogs;
use super::ffprocesschildwrapper::FFProcessChildWapper;
use super::ffstatus::FFStatus;

enum Message {
    Refresh,
    Quit,
    UpdateStatus,
}

pub struct FFProcessHolder {
    process: FFProcess,
}

#[derive(Clone, Debug)]
pub struct FFProcess {
    sender: mpsc::Sender<Message>,
}

struct FFProcessActor {
    server_info: Server,
    config_uid: String,
    receiver: mpsc::Receiver<Message>,
    sender: mpsc::Sender<Message>,
    process: Option<FFProcessChildWapper>,
    std_out_handle: Option<JoinHandle<()>>,
    std_err_handle: Option<JoinHandle<()>>,
    sys_info_handle: Option<JoinHandle<()>>,
    ffdb: FFDb,
    base_output: std::path::PathBuf,
    ffmpeg: String,
    modified_ffmpeg: bool,
    status: Arc<FFStatus>,
    logs: FFLogs,
}

impl FFProcessHolder {
    pub fn new(
        server_info: Server,
        config_uid: String,
        ffdb: FFDb,
        base_output: std::path::PathBuf,
        ffmpeg: String,
        modified_ffmpeg: bool,
    ) -> Self {
        Self {
            process: FFProcess::new(
                server_info,
                config_uid,
                ffdb,
                base_output,
                ffmpeg,
                modified_ffmpeg,
            ),
        }
    }

    pub fn get(&self) -> FFProcess {
        self.process.clone()
    }
}

impl Drop for FFProcessHolder {
    fn drop(&mut self) {
        self.process.quit();
    }
}

impl FFProcess {
    fn new(
        server_info: Server,
        config_uid: String,
        ffdb: FFDb,
        base_output: std::path::PathBuf,
        ffmpeg: String,
        modified_ffmpeg: bool,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(10);
        let status = Arc::new(FFStatus::new(config_uid.clone()));
        let actor = FFProcessActor::new(
            server_info,
            config_uid,
            receiver,
            sender.clone(),
            ffdb,
            base_output,
            ffmpeg,
            status.clone(),
            modified_ffmpeg,
        );
        tokio::spawn(FFProcessActor::run(actor));

        Self { sender }
    }

    pub async fn refresh(&self) {
        self.sender.send(Message::Refresh).await.ok();
    }

    fn quit(&self) {
        self.sender.try_send(Message::Quit).ok();
    }
}

impl FFProcessActor {
    fn new(
        server_info: Server,
        config_uid: String,
        receiver: mpsc::Receiver<Message>,
        sender: mpsc::Sender<Message>,
        ffdb: FFDb,
        base_output: std::path::PathBuf,
        ffmpeg: String,
        status: Arc<FFStatus>,
        modified_ffmpeg: bool,
    ) -> Self {
        let update_sender = sender.clone();
        tokio::spawn(async move {
            loop {
                match update_sender.send(Message::UpdateStatus).await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("Fail to send update status: {}", e);
                        return;
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        let logs = FFLogs::new(ffdb.clone(), config_uid.clone(), 60, 120, 300);
        Self {
            server_info,
            config_uid,
            receiver,
            sender,
            process: None,
            std_out_handle: None,
            std_err_handle: None,
            sys_info_handle: None,
            ffdb,
            base_output,
            ffmpeg,
            status,
            logs,
            modified_ffmpeg,
        }
    }

    async fn stop(&mut self) {
        if let Some(std_out_handle) = self.std_out_handle.take() {
            std_out_handle.abort();
        }
        if let Some(std_err_handle) = self.std_err_handle.take() {
            std_err_handle.abort();
        }
        if let Some(sys_info_handle) = self.sys_info_handle.take() {
            sys_info_handle.abort();
        }
        if let Some(process) = self.process.take() {
            process.kill().await;
        }

        if self.status.get_state() != model::corestate::CoreState::Stopped {
            self.logs.push("[info] Live Encoder is stopped").await;
            self.status.set_state(model::corestate::CoreState::Stopped);
        }
        tracing::info!("Live Encoder is stopped");
    }

    async fn start(&mut self) -> std::io::Result<()> {
        let config = self
            .ffdb
            .get_ff_config_mut(&self.server_info.uid, &self.config_uid)
            .await;
        if config.is_err() {
            self.logs.push("[error] Config not found").await;
            return Err(new_io_error!(
                std::io::ErrorKind::NotFound,
                "Config not found"
            ));
        }
        let config = config.unwrap();
        if config.state == state::State::Inactive {
            self.status.clear_restart();
            return Ok(());
        }

        if config.input.uri.is_empty() {
            self.logs.push("[error] Input URI is empty").await;
            self.status.set_state(model::corestate::CoreState::Error);
            return Err(new_io_error!(
                std::io::ErrorKind::InvalidInput,
                "Input URI is empty"
            ));
        }

        let gpu = self.server_info.get_gpu(&config.gpu_uid);
        if gpu.is_none() {
            self.logs.push("[error] GPU not found").await;
            self.status.set_state(model::corestate::CoreState::Error);
            return Err(new_io_error!(std::io::ErrorKind::NotFound, "GPU not found"));
        }

        let command = FFCommand::new(
            gpu.as_ref().unwrap(),
            &config,
            &self.base_output,
            self.modified_ffmpeg,
        );
        tracing::info!("Starting ffmpeg: {}", command);
        self.logs.push("[info] Starting Live Encoder").await;

        if let Err(e) = std::fs::create_dir_all(command.output_folder()) {
            tracing::error!("Failed to create output folder: {}", e);
            self.logs
                .push("[error] Failed to create output folder")
                .await;
            self.status.set_state(model::corestate::CoreState::Error);
        }

        let mut process = match Command::new(&self.ffmpeg)
            .args(command.args())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(false)
            .spawn()
        {
            Ok(p) => p,
            Err(e) => {
                self.logs.push("[error] Fail start ffmpeg").await;
                self.status.set_state(model::corestate::CoreState::Error);
                return Err(e);
            }
        };

        self.status.set_state(model::corestate::CoreState::Waiting);
        if let Some(stdout) = process.stdout.take() {
            self.run_std_out(stdout);
        }

        if let Some(stderr) = process.stderr.take() {
            self.run_std_err(stderr);
        }

        if let Some(pid) = process.id() {
            self.run_sys_info(pid);
        }

        self.process = Some(FFProcessChildWapper::new(process));
        Ok(())
    }

    fn run_std_err(&mut self, stderr: ChildStderr) {
        let mut logs = self.logs.clone();
        let std_err_handle = tokio::spawn(async move {
            let reader: tokio::io::BufReader<ChildStderr> = tokio::io::BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                logs.push(&line).await;
            }
        });
        self.std_err_handle = Some(std_err_handle);
    }

    fn run_sys_info(&mut self, pid: u32) {
        assert!(self.sys_info_handle.is_none());
        let status = self.status.clone();
        let pid = sysinfo::Pid::from(pid as usize);
        let sys_info_handle = tokio::spawn(async move {
            let mut s = sysinfo::System::new_all();
            loop {
                s.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&[pid]),
                    false,
                    ProcessRefreshKind::nothing().with_cpu().with_memory(),
                );

                if let Some(process) = s.process(pid) {
                    let cpu = (process.cpu_usage() * 100.0) as u32;
                    let mem = process.memory();
                    status.set_cpu_usage(cpu);
                    status.set_memory_usage(mem)
                } else {
                    return;
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });
        self.sys_info_handle = Some(sys_info_handle);
    }

    fn run_std_out(&mut self, stdout: ChildStdout) {
        assert!(self.std_out_handle.is_none());
        let status: Arc<FFStatus> = self.status.clone();
        let sender = self.sender.clone();
        let mut logs = self.logs.clone();
        let std_out_handle = tokio::spawn(async move {
            let mut is_running = false;
            let reader = tokio::io::BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if line.starts_with("avg_speed=") {
                    let speed = line.split("avg_speed=").collect::<Vec<&str>>()[1];
                    let speed = &speed[0..speed.len() - 1].trim();
                    let speed = speed.parse().unwrap_or(0.0);

                    if speed > 0.0 && !is_running {
                        logs.push("[info] Live Encoder is running").await;
                        status.set_state(model::corestate::CoreState::Running);
                        is_running = true;
                    } else if speed == 0.0 && is_running {
                        logs.push("[warning] Waiting...").await;
                        status.set_state(model::corestate::CoreState::Waiting);
                        is_running = false;
                    }

                    status.set_speed(speed);
                } else if line.starts_with("drop_frames=") {
                    let drop_frames = line.split("drop_frames=").collect::<Vec<&str>>()[1];
                    status.set_drop_frames(drop_frames.parse().unwrap_or(0));
                } else if line.starts_with("dup_frames=") {
                    let dup = line.split("dup_frames=").collect::<Vec<&str>>()[1];
                    status.set_duplicate_frames(dup.parse().unwrap_or(0));
                } else if line.starts_with("out_time_us=") {
                    let out_time_us = line.split("out_time_us=").collect::<Vec<&str>>()[1];
                    status.set_out_time_ms(out_time_us.parse().unwrap_or(0) / 1000);
                } else if line.starts_with("avg_fps=") {
                    let fps = line.split("avg_fps=").collect::<Vec<&str>>()[1].trim();
                    let fps = fps.parse().unwrap_or(0.0);
                    status.set_fps(fps);
                } else if line.starts_with("progress=") && line.ends_with("end") {
                    logs.push("[error] end of the stream...").await;
                    status.set_state(model::corestate::CoreState::Error);
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    status.inc_restart();
                    sender.send(Message::Refresh).await.ok();
                    is_running = false;
                }
                tracing::debug!("{}", line);
            }

            logs.push("[error] ffmpeg is stopped").await;
            status.set_state(model::corestate::CoreState::Error);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            status.inc_restart();
            sender.send(Message::Refresh).await.ok();
        });
        self.std_out_handle = Some(std_out_handle);
    }

    async fn update_status(&mut self) {
        let status = self.status.get_status();
        self.ffdb.set_ff_status(status, 10).await.ok();
    }

    async fn run(mut actor: Self) {
        while let Some(msg) = actor.receiver.recv().await {
            match msg {
                Message::Refresh => {
                    actor.stop().await;
                    actor.start().await.ok();
                }
                Message::Quit => {
                    actor.stop().await;
                    return;
                }
                Message::UpdateStatus => {
                    actor.update_status().await;
                }
            }
        }
    }
}
