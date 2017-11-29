#![feature(unique)]
#![feature(allocator_api)]

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