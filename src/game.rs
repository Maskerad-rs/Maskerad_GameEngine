use core::clock::Clock;
use std::thread;
use std::time::Duration;

/*
    The main struct :

    Will take care of the game loop
*/

pub struct Game {
    game_over: bool,
    game_clock: Clock,
    preferred_frame_rate_millisec: u64,
}

impl Game {
    fn new() -> Self {
        Game {
            game_over: false,
            game_clock: Clock::new(),
            preferred_frame_rate_millisec: 16, //16ms ~= 0.016 s ~= 60fps
        }
    }

    fn run(&mut self) {

        //preferred delta time
        let mut dt = self.preferred_frame_rate_millisec;
        let mut begin_tick = Clock::get_current_time();
        let mut end_tick = Clock::get_current_time();

        while !self.game_over {
            //update the clock
            self.game_clock.update(dt);

            //Update over systems...

            end_tick = Clock::get_current_time();
            dt = Clock::calculate_delta_time_milliseconds(begin_tick, end_tick);
            begin_tick = end_tick;

            //sleep for (preferred frame rate - delta time)
            thread::sleep(Duration::from_millis(self.preferred_frame_rate_millisec - dt));
        }
    }
}