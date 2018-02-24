// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;

use std::path::{Path, PathBuf};
use std::env;
use filesystem::filesystem_error::{FileSystemError, FileSystemResult};
use std::fmt;

//Enum used to specify the 'root' directory from where to write/delete/open dir/files
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum RootDir {
    WorkingDirectory,
    UserDataRoot,
    UserConfigRoot,
    EngineConfigRoot,
    EngineLogRoot,
    UserSaveRoot,
}

impl fmt::Display for RootDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RootDir::WorkingDirectory => {
                write!(f, "current directory")
            },
            &RootDir::UserDataRoot => {
                write!(f, "user data root")
            },
            &RootDir::UserConfigRoot => {
                write!(f, "user config root")
            },
            &RootDir::EngineConfigRoot => {
                write!(f, "engine config root")
            },
            &RootDir::EngineLogRoot => {
                write!(f, "engine log root")
            },
            &RootDir::UserSaveRoot => {
                write!(f, "user save root")
            },
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameDirectories(HashMap<RootDir, PathBuf>);

impl GameDirectories {
    pub fn new<S>(game_name: S, game_author: S) -> FileSystemResult<Self> where
        S: AsRef<str>
    {
        debug!("Creating a new GameDirectories with a game name of {}, created by {}", game_name.as_ref(), game_author.as_ref());
        trace!("Creating the user config path...");
        let mut user_config = PathBuf::new();
        trace!("Creating the user data path...");
        let mut user_data = PathBuf::new();

        if cfg!(target_os = "windows") {
            trace!("OS: Windows.");
            trace!("Trying to get the value of the APPDATA environment variable.");
            let appdata = env::var("APPDATA")?;

            user_config = PathBuf::from(format!("{}\'{}\'{}", appdata.as_str(), game_author.as_ref(), game_name.as_ref()));
            user_data = PathBuf::from(format!("{}\'{}\'{}", appdata.as_str(), game_author.as_ref(), game_name.as_ref()));
        } else if cfg!(target_os = "macos") {
            trace!("OS: MacOS.");
            unimplemented!();
        } else {
            trace!("OS: Unix/Linux/BSD.");
            trace!("Trying to get the value of the HOME environment variable.");
            let home = env::var("HOME")?;

            user_config = PathBuf::from(format!("{}/.config/{}/{}", home.as_str(), game_author.as_ref(), game_name.as_ref()));
            user_data = PathBuf::from(format!("{}/.local/share/{}/{}", home.as_str(), game_author.as_ref(), game_name.as_ref()));
        }

        trace!("User config path: {}", user_config.display());
        trace!("User data path: {}", user_data.display());


        let mut logs = user_config.clone();
        logs.push("maskerad_logs");
        trace!("engine logs path: {}", logs.display());

        let mut engine_config = user_config.clone();
        engine_config.push("maskerad_configuration");
        trace!("engine configuration path: {}", engine_config.display());

        let mut saves = user_data.clone();
        saves.push("game_saves");
        trace!("game saves path: {}", saves.display());

        trace!("Trying to get the path of the current directory...");
        let current = env::current_dir()?;
        trace!("Current directory: {}", current.display());

        trace!("Creating the hashmap associating the RootDir enumeration to those paths.");
        let mut directories = HashMap::with_capacity(6);
        directories.insert(RootDir::WorkingDirectory, current);
        directories.insert(RootDir::UserDataRoot, user_data);
        directories.insert(RootDir::UserConfigRoot, user_config);
        directories.insert(RootDir::EngineConfigRoot, engine_config);
        directories.insert(RootDir::EngineLogRoot, logs);
        directories.insert(RootDir::UserSaveRoot, saves);
        trace!("GameDirectories structure successfully created.");
        Ok(GameDirectories(directories))
    }

    pub fn get(&self, k: &RootDir) -> Option<&Path> {
        match self.0.get(k) {
            Some(pathbuf) => {
                Some(pathbuf.as_path())
            },
            None => {
                None
            }
        }
    }
}
