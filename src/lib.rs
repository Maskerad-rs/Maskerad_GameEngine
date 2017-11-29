//TODO: a private module 'dev_internal' ? to hide the 3 crate and the game module ?

extern crate blacksmith_core;
extern crate blacksmith_subsystems;
extern crate blacksmith_gameplay;

pub mod game;

pub mod core {
    pub use blacksmith_core::engine_support_systems::system_interfaces;
    pub use blacksmith_core::engine_config as engine_configuration;
    pub use blacksmith_core::game_infos::GameInfos;
    pub use blacksmith_core::game_directories::GameDirectories;
    pub use blacksmith_core::clock::Clock;
}

pub mod subsystems {
    pub use blacksmith_subsystems::platforms;
}

pub mod gameplay {

}




