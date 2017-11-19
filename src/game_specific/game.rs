use core::clock::Clock;
use std::thread;
use std::time::Duration;

/*
    The main struct :

    Will take care of the game loop
*/

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

        //Read config files

        //Create systems builders according to the config read. (platform, number of threads...).

        //create our multithreaded ref counted immutable systems (Arc<OurSystem>)

        //create the singleton system threadpools.
        //Our objects will send messages to those system threadpools.
        //Those threadpools will repackages those messages to send
        //messages to their workers.
        //Those threadpools, when created, contain a 'reference' to the systems.
        //When sending a message to their workers, those threadpools will
        //package the message AND a 'reference' to the system.


        //Prepare the gameloop

        //preferred delta time
        let mut delta = self.preferred_frame_rate_millisec;
        let mut previous_time = Clock::get_current_time();
        let mut current_time = Clock::get_current_time();
        let mut lag = 0.0;

        while !self.game_over {
            current_time = Clock::get_current_time();

            delta = Clock::calculate_delta_time_seconds(previous_time, current_time);
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
    }
}