// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod random;
pub mod engine_config;
pub mod engine_config_error;

extern crate toml;

extern crate cgmath;
extern crate rand;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate maskerad_independence_layer;

#[cfg(test)]
mod maskerad_core_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2);
    }
}