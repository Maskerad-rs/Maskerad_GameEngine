// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad_gameplay_foundations;

extern crate maskerad_front_end;
extern crate maskerad_visual_effects;
extern crate maskerad_rendering_optimizations;
extern crate maskerad_low_level_renderer;
extern crate maskerad_animations;
extern crate maskerad_physics;

extern crate maskerad_inputs;
extern crate maskerad_audio;
extern crate maskerad_network;
extern crate maskerad_debugging;

extern crate maskerad_resource_management;

extern crate maskerad_core;
extern crate maskerad_independence_layer;

#[cfg(test)]
mod maskerad_game_engine_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2);
    }
}

