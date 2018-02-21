// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use time::{Duration, PreciseTime};

//A simple clock class, which can represent a real_time clock, a game clock or a special clock for an
//animation system for example.

//It stores the total time elapsed since it was created with a Duration type.

pub struct Clock {
    total_time: Duration,
    paused: bool,
    time_scale: f64,
    framerate_single_step_ms: Duration,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            total_time: Duration::milliseconds(0),
            paused: false,
            time_scale: 1.0,
            framerate_single_step_ms: Duration::milliseconds(16), //16ms == 0.016s == 1/60 == 60Hz.
        }
    }
}

impl Clock {
    /// create a new non-paused clock, with the timer initialized to 0 and a time scale of 1.0
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_independence_layer::clock::Clock;
    ///
    /// let clock = Clock::new();
    /// assert!(!clock.is_paused());
    /// assert_eq!(clock.time_scale(), 1.0);
    /// assert_eq!(clock.total_time_ms(), 0);
    ///
    /// ```
    pub fn new() -> Self {
        Default::default()
    }


    pub fn with_single_step(framerate_single_step_ms: Duration) -> Self {
        Clock {
            total_time: Duration::milliseconds(0),
            paused: false,
            time_scale: 1.0,
            framerate_single_step_ms,
        }
    }

    //Duration is Copy.
    pub fn update(&mut self, delta_time: Duration) {
        //If the clock is "paused", we don't update the total time.
        if !self.paused {
            let mill_to_sec = delta_time.num_milliseconds() as f64 * 1e-3;
            let scaled_sec = mill_to_sec * self.time_scale;
            let scaled_duration = Duration::milliseconds((scaled_sec * 1e3) as i64);
            let current_total_time = self.total_time;

            self.total_time = current_total_time + scaled_duration;
        }
    }

    pub fn total_time_ms(&self) -> i64 {
        self.total_time.num_milliseconds()
    }

    //PreciseTime is Copy.
    //PreciseTime is a wrapper around a u64. And PreciseTime::now() is precise_time_ns() under the hood.
    pub fn current_time() -> PreciseTime {
        PreciseTime::now()
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
            let mill_to_sec = self.framerate_single_step_ms.num_milliseconds() as f64 * 1e-3;
            let scaled_sec = mill_to_sec * self.time_scale;
            let scaled_duration = Duration::milliseconds((scaled_sec * 1e3) as i64);
            let current_total_time = self.total_time;

            self.total_time = current_total_time + scaled_duration;
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
        assert!(clock.total_time_ms() >= 16);
        clock.single_step();
        assert!(clock.total_time_ms() >= 32);
    }

    #[test]
    fn clock_calculate_dt_second() {
        let mut clock = Clock::new();
        let old_time = Clock::current_time();
        thread::sleep(Duration::seconds(1).to_std().unwrap());
        let new_time = Clock::current_time();
        let dt = old_time.to(new_time);
        assert!(dt.num_milliseconds() >= 1000);
    }

    #[test]
    fn clock_update() {
        let mut clock = Clock::new();
        clock.update(Duration::milliseconds(16)); // + 0.016
        clock.set_time_scale(2.0);
        clock.update(Duration::milliseconds(16)); // + 0.032
        assert!(clock.total_time_ms() >= 48);
    }
}