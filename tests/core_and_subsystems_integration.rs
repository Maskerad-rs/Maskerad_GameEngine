extern crate blacksmith;

use blacksmith::systems::linux::filesystem::Filesystem;
use blacksmith::system_interfaces::filesystems::{VFilesystem, RootDir};

use blacksmith::GameInfos;
use blacksmith::engine_configuration::{DebugOptions, EngineConfig};

#[test]
fn engine_config_load_save_config() {

    let game_infos = GameInfos::new("test_integration_core_subsystems", "Malkaviel");
    let filesystem = Box::new(Filesystem::new(game_infos).unwrap()) as Box<VFilesystem>;
    let engine_configuration = EngineConfig::new(DebugOptions::new(false));
    engine_configuration.save_config(&filesystem);

    let new_engine_configuration = EngineConfig::load_config(&filesystem).unwrap();
    assert!(!new_engine_configuration.debug_options.flush);
    //cleanup.
    filesystem.rmrf(RootDir::UserDataRoot, "").unwrap();
    filesystem.rmrf(RootDir::UserConfigRoot, "").unwrap();
}