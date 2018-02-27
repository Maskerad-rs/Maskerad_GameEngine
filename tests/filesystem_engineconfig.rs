// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad_game_engine;

//Filesystem stuff.
use maskerad_game_engine::core::filesystem::filesystem::Filesystem;
use maskerad_game_engine::core::filesystem::game_directories::RootDir;

//EngineConfig stuff.
use maskerad_game_engine::core::engine_configuration::engine_config::EngineConfig;

use std::io::BufReader;

#[test]
fn serialize_deserialize_engineconfig() {
    let filesystem = Filesystem::new("test_ser_deser_engine_config", "Malkaviel")
        .expect(format!("{}::{} Could not create FS", file!(), line!()).as_str());

    //Deserialization
    let path =
        filesystem.construct_path_from_root(RootDir::WorkingDirectory, "test_resources/engine_configuration/test_config.toml")
            .expect(format!("{}::{} Could not create path", file!(), line!()).as_str());


    let mut reader = Filesystem::open(path.as_path())
        .expect(format!("{}::{} Could not create the BufReader", file!(), line!()).as_str());

    let engine_config = EngineConfig::from_reader(&mut reader)
        .expect(format!("{}::{} Could not create the engineconfig from the reader", file!(), line!()).as_str());

    assert!(engine_config.script_path().is_some());
    assert_eq!(engine_config.locale(), "EN");

    //Serialization
    let ser_path = filesystem.construct_path_from_root(RootDir::WorkingDirectory, "test_resources/engine_configuration/test_ser_config.toml")
        .expect(format!("{}::{} Could not create path", file!(), line!()).as_str());

    let ser_config = EngineConfig::new("FR", None);

    let mut writer = Filesystem::create(ser_path.as_path()).expect(format!("{}::{} Could not create file", file!(), line!()).as_str());
    ser_config.save_to_toml(&mut writer).expect(format!("{}::{} Could not serialize config", file!(), line!()).as_str());

    assert!(ser_path.exists());
}