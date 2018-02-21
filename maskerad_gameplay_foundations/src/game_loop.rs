// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
    Game programming patterns solution: Fixed update for physic and AI, variable update for rendering.
    Game engine architecture: networked game loop.
*/

/*

use maskerad_independence_layer::clock::Clock;
use std::thread;
use std::time::Duration;

pub struct Game {
    game_over: bool,
    preferred_frame_rate_millisec: f64,
}

impl Game {
    fn new() -> Self {
        Game {
            game_over: false,
            preferred_frame_rate_millisec: 0.016, //16ms ~= 0.016 s ~= 60fps
        }
    }

    fn run(&mut self) {

        //create the systems...
        let mut game_clock = Clock::new();

        //Read config files (saved options or something like that)

        //create a rayon's user-created threadpool ? to use install() ?
        //Should gameobjects own a Rc<Threadpool> ? When world call update on the object, the object call update to its components and pass a clone of this threadpool

        //create our systems( only the cpu intensive systems should use the threadpool, not a filesystem...

        //Prepare the gameloop

        //preferred delta time
        let mut delta = self.preferred_frame_rate_millisec;
        let mut previous_time = game_clock.current_time();
        let mut current_time = game_clock.current_time();
        let mut lag = 0.0;

        while !self.game_over {
            current_time = game_clock.current_time();

            delta = game_clock.calculate_delta_time_seconds(previous_time, current_time);
            previous_time = current_time;
            lag += delta;

            //update the clock (stock the duration since the game has been running)
            game_clock.update(delta);

            //Get input from keyboard/mouse/gamepad...
            //input system -> get input -> retrieve input events from a queue ?

            //We update physic, AI, ... with a fixed time step (120 fps for example).
            //Physic system needs a stable integration step to not blow up completely.
            while lag >= 0.0012 {
                //updates objects contained in the world, and delete those marked for deletion
                //world structure -> update(delta) -> call update(delta) on each object.

                lag -= 0.0012;
            }

            //Shouldn't we .join() every worker of each systems here ?
            //
            //
            //


            //WE MIGHT HAVE PROBLEMS WITH RENDERING AND MULTITHREADING ->
            //STUFF MUST BE DRAWN FROM THE FARTHEST OBJECT TO THE CLOSEST, RIGHT ?

            //We update the rendering at a semi-variable time step (constant 60 fps if we can, less otherwise).
            //draw each gameobject
            //world structure -> draw(lag / 0.0012) -> call draw on each object. (see p132 for lag / 0.0012)

            //display then clear the screen with a color
            //display_system -> swap buffers

            //sleep for (preferred frame rate - delta time). 'Guarantee' a consistent framerate.
            thread::sleep(Duration::from_millis(((self.preferred_frame_rate_millisec - delta) * 1e3) as u64));
        }

        //Out of the game loop, time to quit !


        //THe rayon threadpool impl the drop trait. Nothing to do the handle the destruction of it.

        //shut down every systems in the inverse order of initialization
    }
}

*/