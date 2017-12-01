// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::PathBuf;
use std::env;
use game_infos::GameInfos;
use engine_support_systems::error_handling::error::{GameError, GameResult};

//A collection of paths used by the game engine's filesystem.
#[derive(Debug)]
pub struct GameDirectories {
    user_config_path: PathBuf,
    user_data_path: PathBuf,
    logs_path: PathBuf,
    engine_configuration_path: PathBuf,
    saves_path: PathBuf,
    current_path: PathBuf,
}

impl GameDirectories {
    pub fn new(game_infos: &GameInfos) -> GameResult<Self> {

        let user_config = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(format!("{}\'{}\'{}", appdata.as_str(), game_infos.author().as_str(), game_infos.game_name().as_str()))
        } else if cfg!(target_os = "macos") {
            unimplemented!();
        } else {
            let home = env::var("HOME")?;
            PathBuf::from(format!("{}/.config/{}", home.as_str(), game_infos.game_name().as_str()))
        };

        let user_data = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(format!("{}\'{}\'{}", appdata.as_str(), game_infos.author().as_str(), game_infos.game_name().as_str()))
        } else if cfg!(target_os = "macos") {
            unimplemented!();
        } else {
            let home = env::var("HOME")?;
            PathBuf::from(format!("{}/.local/share/{}", home.as_str(), game_infos.game_name().as_str()))
        };

        let mut logs = user_config.clone();
        logs.push("blacksmith_logs");
        let mut engine_config = user_config.clone();
        engine_config.push("blacksmith_configuration");
        let mut saves = user_data.clone();
        saves.push("game_saves");
        let current = env::current_dir()?;

        Ok(GameDirectories {
            user_config_path: user_config,
            user_data_path: user_data,
            logs_path: logs,
            engine_configuration_path: engine_config,
            saves_path: saves,
            current_path: current,
        })
    }

    pub fn user_data_path(&self) -> &PathBuf {
        &self.user_data_path
    }

    pub fn user_config_path(&self) -> &PathBuf {
        &self.user_data_path
    }

    pub fn logs_path(&self) -> &PathBuf {&self.logs_path}

    pub fn engine_configuration_path(&self) -> &PathBuf {&self.engine_configuration_path}

    pub fn saves_path(&self) -> &PathBuf {&self.saves_path}

    pub fn current_path(&self) -> &PathBuf {&self.current_path}


}