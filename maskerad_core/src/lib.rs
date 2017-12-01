// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod engine_support_systems;
pub mod engine_config;
pub mod random;
pub mod clock;
pub mod maths;
pub mod game_infos;
pub mod game_directories;

extern crate cgmath;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;