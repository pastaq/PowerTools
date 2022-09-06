use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::settings::{OnResume, Settings};
use crate::utility::unwrap_maybe_fatal;

const ALLOWED_ERROR: f64 = 0.001;

pub fn spawn(settings: Settings) -> JoinHandle<()> {
    thread::spawn(move || {
        log::info!("resume_worker starting...");
        let duration = Duration::from_millis(5000);
        let mut start = Instant::now();
        loop {
            thread::sleep(duration);
            let old_start = start.elapsed();
            start = Instant::now();
            if old_start.as_secs_f64() > duration.as_secs_f64() * (1.0 + ALLOWED_ERROR) {
                // has just resumed from sleep
                unwrap_maybe_fatal(settings.on_resume(), "On resume failure");
                log::debug!(
                    "OnResume completed after sleeping for {}s",
                    old_start.as_secs_f32()
                );
            }
        }
        //log::warn!("resume_worker completed!");
    })
}