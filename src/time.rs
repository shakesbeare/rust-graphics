use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

static T_PROGRAM_START: OnceLock<Instant> = OnceLock::new();
static T_LAST_FRAME: OnceLock<RwLock<Instant>> = OnceLock::new();

pub fn startup() {
    let ps_res = T_PROGRAM_START.set(Instant::now());
    let lf_res = T_LAST_FRAME.set(RwLock::new(Instant::now()));

    if ps_res.is_err() {
        log::error!("Could not set program start time");
    }

    if lf_res.is_err() {
        log::error!("Could not initialize last frame time");
    }
}

pub fn update() {
    if let Some(rw_lock) = T_LAST_FRAME.get() {
        let can_acquire = rw_lock.write();
        match can_acquire {
            Ok(mut last_frame) => {
                *last_frame = Instant::now();
            }
            Err(e) => {
                log::error!("Could not update the frame time, {}", e);
            }
        }
    }
}

pub fn total_elapsed() -> Duration {
    let Some(prog_start) = T_PROGRAM_START.get() else {
        log::debug!("T_PROGRAM_START was not initialized");
        return Duration::ZERO;
    };
    prog_start.elapsed()
}

pub fn delta_time() -> f32 {
    let last_frame = T_LAST_FRAME.get().unwrap().read().unwrap();
    last_frame.elapsed().as_secs_f32()
}
