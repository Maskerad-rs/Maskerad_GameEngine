// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use serde_derive;

use std::io::BufReader;
use std::io::Read;

use engine_support_systems::error_handling::error::{GameResult, GameError};
use engine_support_systems::system_interfaces::filesystems::VFilesystem;
use engine_support_systems::system_interfaces::filesystems::RootDir;
//TODO: EngineConfig need a default impl with default mapping of options.

//Here what the file should look like :

/*
[graphic]
key = value

[physic]
...

[debug]
flush = true/false

[input]
"move_up = "Z"
...
*/



#[derive(Deserialize, Serialize, Debug)]
pub struct EngineConfig {
    pub debug_options: DebugOptions,
}
impl EngineConfig {
    pub fn new(debug_options: DebugOptions) -> Self {
        EngineConfig {
            debug_options,
        }
    }

    //Deserialize a toml string to an EngineConfig rust structure.
    pub fn load_config(filesystem: &Box<VFilesystem>) -> GameResult<Self> {
        let mut bufreader = BufReader::new(filesystem.open(RootDir::UserEngineConfigurationRoot, "engine_configuration.toml")?);
        let mut toml_string = String::new();
        bufreader.read_to_string(&mut toml_string)?;
        Ok(toml::from_str(toml_string.as_str())?)

    }

    //Serialize a EngineConfig to TOML string, to be saved in a toml file.
    pub fn save_config(&self, filesystem: &Box<VFilesystem>) -> GameResult<()> {
        let toml_string = toml::to_string(&self)?;

        //override the existing engine_configuration.toml
        filesystem.create(RootDir::UserEngineConfigurationRoot, "engine_configuration.toml")?.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}




#[derive(Deserialize, Serialize, Debug)]
pub struct DebugOptions {
    pub flush: bool,
}
impl DebugOptions {
    pub fn new(flush: bool) -> Self {
        DebugOptions {
            flush,
        }
    }
}

//TODO: EngineConfig tests
#[cfg(test)]
mod engine_config_test {
    use super::*;

    #[test]
    fn engine_config_serialization_deserialization() {
        let engine_configuration = EngineConfig::new(DebugOptions::new(false));
        assert!(!engine_configuration.debug_options.flush);
        //Serialize the EngineConfig rust structure to a TOML string.
        let toml = toml::to_string(&engine_configuration).unwrap();

        //Deserialize the TOML string to an EngineConfig rust structure.
        let new_engine_configuration: EngineConfig = toml::from_str(toml.as_str()).unwrap();
        assert!(!new_engine_configuration.debug_options.flush)

    }
}