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
    ///create a new non-paused clock, with the timer initialized to 0 and a time scale of 1.0
    /// # Example
    ///
    /// ```
    /// let clock = blacksmith_core::clock::Clock::new();
    /// assert!(!clock.is_paused());
    /// assert_eq!(clock.time_scale(), 1.0);
    /// assert_eq!(clock.duration(), std::time::Duration::new(0, 0));
    /// ```
    pub fn new() -> Self {
        Clock {
            total_time: Duration::new(0, 0),
            paused: false,
            time_scale: 1.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if !self.paused {
            let scaled_time = (delta_time as f64 * self.time_scale) as f64;

            self.total_time += Duration::from_millis((scaled_time * 1e3) as u64);
        }
    }

    pub fn total_time_seconds(&self) -> f64 {
        self.total_time.as_secs() as f64 + (self.total_time.subsec_nanos() as f64 * 1e-9) as f64
    }

    pub fn duration(&self) -> Duration {
        self.total_time
    }

    pub fn current_time(&self) -> Instant {
        Instant::now()
    }

    pub fn calculate_delta_time_seconds(&self, begin_tick: Instant, end_tick: Instant) -> f64 {
        let duration = end_tick.duration_since(begin_tick);
        duration.as_secs() as f64 + (duration.subsec_nanos() as f64 * 1e-9) as f64
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale;
    }

    pub fn single_step(&mut self) {
        if self.paused {
            let scaled_time = (0.016 * self.time_scale) as f64;
            self.total_time += Duration::from_millis((scaled_time * 1e3) as u64);
        }
    }
}

#[cfg(test)]
mod clock_tests {
    use super::*;
    use std::thread;

    #[test]
    fn clock_creation_and_accessors() {
        let mut clock = Clock::new();
        assert!(!clock.is_paused());
        clock.set_paused(true);
        assert!(clock.is_paused());
        clock.set_paused(false);

        assert_eq!(clock.time_scale(), 1.0);
        clock.set_time_scale(2.0);
        assert_eq!(clock.time_scale(), 2.0);

    }

    #[test]
    fn clock_single_step() {
        let mut clock = Clock::new();
        clock.set_paused(true);
        clock.single_step();
        assert!(clock.total_time_seconds() >= 0.016);
        clock.single_step();
        assert!(clock.total_time_seconds() >= 0.032);
    }

    #[test]
    fn clock_calculate_dt_second() {
        let mut clock = Clock::new();
        let old_time = clock.current_time();
        thread::sleep(Duration::from_secs(1));
        let new_time = clock.current_time();
        let dt = clock.calculate_delta_time_seconds(old_time, new_time);
        assert!(dt >= 1.0);
    }

    #[test]
    fn clock_update() {
        let mut clock = Clock::new();
        clock.update(0.016); // + 0.016
        clock.set_time_scale(2.0);
        clock.update(0.016); // + 0.032
        assert!(clock.total_time_seconds() >= 0.048);
    }
}