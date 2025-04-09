use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};

use crate::model::{self, corestate::CoreState};

#[derive(Debug)]
pub struct FFStatus {
    pub config_uid: String,
    pub speed: AtomicU64,
    pub drop_frames: AtomicU64,
    pub duplicate_frames: AtomicU64,
    pub out_time_ms: AtomicU64,
    pub fps: AtomicU64,
    pub state: AtomicU8,
    pub cpu_usage: AtomicU32,
    pub memory_usage: AtomicU64,
    pub nb_restart: AtomicU32,
}

impl FFStatus {
    pub fn new(config_uid: String) -> Self {
        Self {
            config_uid,
            speed: AtomicU64::new(0),
            drop_frames: AtomicU64::new(0),
            duplicate_frames: AtomicU64::new(0),
            out_time_ms: AtomicU64::new(0),
            fps: AtomicU64::new(0),
            state: AtomicU8::new(model::corestate::CoreState::Stopped as u8),
            cpu_usage: AtomicU32::new(0),
            memory_usage: AtomicU64::new(0),
            nb_restart: AtomicU32::new(0),
        }
    }

    pub fn set_memory_usage(&self, memory_usage: u64) {
        self.memory_usage.store(memory_usage, Ordering::Relaxed);
    }

    pub fn set_cpu_usage(&self, cpu_usage: u32) {
        self.cpu_usage.store(cpu_usage, Ordering::Relaxed);
    }

    pub fn set_speed(&self, speed: f64) {
        self.speed.store((speed * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn set_drop_frames(&self, drop_frames: u64) {
        self.drop_frames.store(drop_frames, Ordering::Relaxed);
    }

    pub fn set_duplicate_frames(&self, dup: u64) {
        self.duplicate_frames.store(dup, Ordering::Relaxed);
    }

    pub fn set_out_time_ms(&self, out_time_ms: u64) {
        self.out_time_ms.store(out_time_ms, Ordering::Relaxed);
    }

    pub fn set_fps(&self, fps: f64) {
        self.fps.store((fps * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn get_state(&self) -> CoreState {
        CoreState::from_u8(self.state.load(Ordering::Relaxed))
    }

    pub fn inc_restart(&self) {
        self.nb_restart.fetch_add(1, Ordering::Relaxed);
    }

    pub fn clear_restart(&self) {
        self.nb_restart.store(0, Ordering::Relaxed);
    }

    pub fn set_state(&self, state: model::corestate::CoreState) {
        if state == model::corestate::CoreState::Stopped {
            self.set_speed(0.0);
            self.set_fps(0.0);
            self.set_cpu_usage(0);
            self.set_memory_usage(0);
            self.set_out_time_ms(0);
            self.set_drop_frames(0);
            self.set_duplicate_frames(0);
        }

        self.state.store(state as u8, Ordering::Relaxed);
    }

    pub fn get_status(&self) -> model::ff::ffstatusvalue::FFStatusValue {
        model::ff::ffstatusvalue::FFStatusValue {
            config_uid: self.config_uid.clone(),
            speed: self.speed.load(Ordering::Relaxed) as f64 / 1000.0,
            drop_frames: self.drop_frames.load(Ordering::Relaxed),
            duplicate_frames: self.duplicate_frames.load(Ordering::Relaxed),
            out_time_ms: self.out_time_ms.load(Ordering::Relaxed),
            fps: self.fps.load(Ordering::Relaxed) as f64 / 1000.0,
            current_state: model::corestate::CoreState::from_u8(self.state.load(Ordering::Relaxed)),
            cpu_usage: self.cpu_usage.load(Ordering::Relaxed),
            memory_usage: self.memory_usage.load(Ordering::Relaxed),
            nb_restart: self.nb_restart.load(Ordering::Relaxed),
        }
    }
}
