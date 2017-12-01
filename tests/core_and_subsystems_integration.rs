// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad;

use maskerad::subsystems::platforms::linux::filesystem::Filesystem;
use maskerad::core::system_interfaces::filesystems::{VFilesystem, RootDir};

use maskerad::core::GameInfos;
use maskerad::core::engine_configuration::{EngineConfig, DebugOptions};

#[test]
fn engine_config_load_save_config() {

    let game_infos = GameInfos::new("test_integration_core_subsystems", "Malkaviel");
    let filesystem = Box::new(Filesystem::new(game_infos).expect("Couldn't create a game_infos struct")) as Box<VFilesystem>;
    let engine_configuration = EngineConfig::new(DebugOptions::new(false));
    engine_configuration.save_config(&filesystem).expect("Coulodn't save the engine config");

    let new_engine_configuration = EngineConfig::load_config(&filesystem).expect("Couldn't load the engine config");
    assert!(!new_engine_configuration.debug_options.flush);

}