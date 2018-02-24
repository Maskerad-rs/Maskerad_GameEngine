// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{Read, Write};
use std::path::PathBuf;
use toml;
use engine_configuration::engine_config_error::{EngineConfigError, EngineConfigResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineConfig {
    language: String,
    script: Option<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            language: String::from("EN"),
            script: None,
        }
    }
}

impl EngineConfig {
    pub fn new<S, C>(language: S, script_path: C) -> Self where
        S: Into<String>,
        C: Into<Option<String>>,
    {
        EngineConfig {
            language: language.into(),
            script: script_path.into(),
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> EngineConfigResult<Self> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        toml::from_str(content.as_str()).map_err(|toml_deser_error| {
            EngineConfigError::from(toml_deser_error)
        })
    }

    pub fn save_to_toml<W: Write>(&self, writer: &mut W) -> EngineConfigResult<()> {
        let string = toml::to_string(&self).map_err(|ser_error| {
            EngineConfigError::from(ser_error)
        })?;
        writer.write_all(string.as_bytes()).map_err(|io_error| {
            EngineConfigError::from(io_error)
        })
    }

    pub fn language(&self) -> &str {
        self.language.as_str()
    }

    pub fn script_path(&self) -> Option<&str> {
        match self.script {
            Some(ref path) => {
                Some(path.as_str())
            },
            None => {
                None
            },
        }
    }

    pub fn set_language<S>(&mut self, lang: S) where
        S: Into<String>
    {
        self.language = lang.into();
    }

    pub fn set_script_path<S>(&mut self, script_path: S) where
        S: Into<Option<String>>
    {
        self.script = script_path.into();
    }
}





#[cfg(test)]
mod engine_configuration_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1+1, 2)
    }
}