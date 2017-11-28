extern crate blacksmith_core;
extern crate blacksmith_subsystems;
extern crate blacksmith_gameplay;

pub mod game;

pub use blacksmith_subsystems::platforms as systems;
pub use blacksmith_core::engine_support_systems::system_interfaces;
pub use blacksmith_core::engine_config as engine_configuration;
pub use blacksmith_core::game_infos::GameInfos;