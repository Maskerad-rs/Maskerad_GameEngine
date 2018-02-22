// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod artificial_intelligence;
pub mod event;
pub mod scripting;
pub mod game_loop;

#[cfg(test)]
mod maskerad_gameplay_foundations_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2);
    }
}