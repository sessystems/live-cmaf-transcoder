#[cfg(target_family = "unix")]
use nix::sys::wait::WaitStatus;
#[cfg(target_family = "unix")]
use nix::{sys::signal, unistd::Pid};
use tokio::process::Child;

pub struct FFProcessChildWapper {
    pub process: Child,
    killed: bool,
}

impl FFProcessChildWapper {
    pub fn new(process: Child) -> Self {
        Self {
            process,
            killed: false,
        }
    }

    #[cfg(target_family = "unix")]
    pub async fn kill(mut self) {
        let pid: i32 = self.process.id().unwrap_or_default() as i32;
        let pid = Pid::from_raw(pid);
        match signal::kill(pid, signal::Signal::SIGTERM) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        };

        self.killed = true;

        loop {
            match nix::sys::wait::waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::Exited(_, sig)) => {
                    tracing::info!("Process {} exited with signal {:?}", pid, sig);
                    break;
                }
                Ok(_) => {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
                Err(e) => {
                    tracing::warn!("PID {}: {}", pid, e);
                    break;
                }
            }
        }
    }

    #[cfg(not(target_family = "unix"))]
    pub async fn kill(self) {}
}

#[cfg(target_family = "unix")]
impl Drop for FFProcessChildWapper {
    fn drop(&mut self) {
        if self.killed {
            return;
        }

        let pid: i32 = self.process.id().unwrap_or_default() as i32;
        let pid = Pid::from_raw(pid);
        tracing::error!("Process {} was not killed", pid);
        let _ = signal::kill(pid, signal::Signal::SIGTERM);
    }
}
