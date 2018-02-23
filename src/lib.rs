// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub extern crate maskerad_gameplay_foundations as gameplay;

pub extern crate maskerad_front_end as gui;
pub extern crate maskerad_visual_effects as vfx;
pub extern crate maskerad_rendering_optimizations as optimizations;
pub extern crate maskerad_low_level_renderer as renderer;
pub extern crate maskerad_animations as animations;
pub extern crate maskerad_physics as physics;

pub extern crate maskerad_inputs as inputs;
pub extern crate maskerad_audio as audio;
pub extern crate maskerad_network as network;
pub extern crate maskerad_debugging as debugging;

pub extern crate maskerad_resource_management as resource_management;
pub extern crate maskerad_core as core;
pub extern crate maskerad_independence_layer as independence_layer;

#[cfg(test)]
mod maskerad_game_engine_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2);
    }
}

