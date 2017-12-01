// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[derive(Debug)]
pub struct GameInfos {
    author: String,
    game_name: String,
}

impl GameInfos {
    pub fn new(game_name: &str, author: &str) -> Self {
        GameInfos {
            author: author.to_string(),
            game_name: game_name.to_string(),
        }
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn game_name(&self) -> String {
        self.game_name.clone()
    }
}