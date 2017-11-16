use std::time::{Instant, Duration};

//A simple clock class, which can represent a real_time clock, a game clock or a special clock for an
//animation system for example.

//It stores the total time elapsed since it was created with a Duration type.

pub struct Clock {
    total_time: Duration,
    paused: bool,
    time_scale: f64,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            total_time: Duration::new(0, 0),
            paused: false,
            time_scale: 1.0,
        }
    }

    pub fn update(&mut self, delta_time: u64) {
        if !self.paused {
            let scaled_time = (delta_time as f64 * self.time_scale) as u64;
            self.total_time += Duration::from_millis(scaled_time);
        }
    }

    pub fn get_total_time_in_milliseconds(&self) -> u64 {
        self.total_time.as_secs() as u64 + (self.total_time.subsec_nanos() as f64 * 1e-3) as u64
    }

    pub fn get_current_time() -> Instant {
        Instant::now()
    }

    pub fn calculate_delta_time_milliseconds(begin_tick: Instant, end_tick: Instant) -> u64 {
        let duration = end_tick.duration_since(begin_tick);
        duration.as_secs() as u64 + (duration.subsec_nanos() as f64 * 1e-3) as u64
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn get_time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale;
    }

    pub fn single_step(&mut self) {
        if self.paused {
            let scaled_time = (16.0 * self.time_scale) as u64;
            self.total_time += Duration::from_millis(scaled_time);
        }
    }
}

//TODO: clock unit tests