// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO: a private module 'dev_internal' ? to hide the 3 crate and the game module ?

extern crate maskerad_core;
extern crate maskerad_subsystems;
extern crate maskerad_gameplay;

pub mod game;

pub mod core {
    pub use maskerad_core::engine_support_systems::system_interfaces;
    pub use maskerad_core::engine_config as engine_configuration;
    pub use maskerad_core::game_infos::GameInfos;
    pub use maskerad_core::game_directories::GameDirectories;
    pub use maskerad_core::clock::Clock;
}

pub mod subsystems {
    pub use maskerad_subsystems::platforms;
}

pub mod gameplay {

}




