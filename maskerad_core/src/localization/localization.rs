// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
 LOCALIZATION SYSTEM.

 The localization system allows you to localize your game, without "too-much" hassle.

 When the engine runs, it will load a configuration object (manually set, or from a toml file),
 which contain the locale used.

 Once we have the locale, we can find the localization file with this path: [CURRENT DIRECTORY]/localization/{locale}/localization.json

 The programmer can then use the Localization to get the correct string, without having to worry
 about the language. The code will not change.
*/

use std::collections::HashMap;
use serde_json;
use std::io::{Read, Write};
use localization::localization_error::{LocalizationError, LocalizationResult};
use std::path::PathBuf;

pub struct Localization {
    manifest: Manifest,
}

impl Localization {
    pub fn from_reader<R: Read>(reader: R) -> LocalizationResult<Self>
    {
        let manifest = Manifest::from_reader(reader)?;

        Ok(Localization {
            manifest
        })
    }

    pub fn get<S>(&self, id: S) -> Option<&str> where
        S: AsRef<str>
    {
        self.manifest.get(id.as_ref())
    }
}

#[derive(Debug, Deserialize)]
struct Manifest(HashMap<String, String>);

impl Manifest {
    pub fn from_reader<R: Read>(reader: R) -> LocalizationResult<Self> {
        serde_json::from_reader(reader).map_err(|json_error| {
            LocalizationError::from(json_error)
        })
    }

    pub fn get<S>(&self, id: S) -> Option<&str> where
        S: AsRef<str>
    {
        match self.0.get(id.as_ref()) {
            Some(string) => {
                Some(string.as_str())
            },
            None => {
                None
            },
        }
    }
}