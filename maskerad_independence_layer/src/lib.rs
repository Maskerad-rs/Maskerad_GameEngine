// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate log;
extern crate remove_dir_all;

extern crate time;
pub mod clock;
pub mod filesystem;

#[cfg(test)]
mod maskerad_independence_layer_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2);
    }
}