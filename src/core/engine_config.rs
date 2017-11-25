use std::env;
use app_dirs;
use toml;
use serde_derive;

use core::engine_support_systems::error_handling::error::{GameResult, GameError};

//TODO: EngineConfig need a default impl with default mapping of options.

#[derive(Deserialize)]
pub struct EngineConfig {

}

impl EngineConfig {
    pub fn new() {
        unimplemented!();
    }

    //Deserialize a toml document into rust structures
    fn load_config(&mut self) -> GameResult<()> {
        unimplemented!();
    }

    fn save_config(&self) -> GameResult<()> {
        unimplemented!();
    }
}