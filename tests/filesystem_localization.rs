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

//Localization stuff.
use maskerad_game_engine::core::localization::localization::Localization;

#[test]
fn load_and_get_translation() {
    let fs = Filesystem::new("test_filesystem_localization", "Malkaviel")
        .expect(format!("{}::{} Could not create the filesystem", file!(), line!()).as_str());

    let path_fr = fs.construct_path_from_root(RootDir::WorkingDirectory, "localization/fr/localization.json")
        .expect(format!("{}::{} Could not create the path to the french translation", file!(), line!()).as_str());
    let path_en = fs.construct_path_from_root(RootDir::WorkingDirectory, "localization/en/localization.json")
        .expect(format!("{}::{} Could not create the path to the english translation", file!(), line!()).as_str());
    let path_es = fs.construct_path_from_root(RootDir::WorkingDirectory, "localization/es/localization.json")
        .expect(format!("{}::{} Could not create the path to the spanish translation", file!(), line!()).as_str());

    let file_fr = Filesystem::open(path_fr.as_path())
        .expect(format!("{}::{} Could not open the file at path {}", file!(), line!(), path_fr.as_path().display()).as_str());
    let file_en = Filesystem::open(path_en.as_path())
        .expect(format!("{}::{} Could not open the file at path {}", file!(), line!(), path_en.as_path().display()).as_str());
    let file_es = Filesystem::open(path_es.as_path())
        .expect(format!("{}::{} Could not open the file at path {}", file!(), line!(), path_es.as_path().display()).as_str());

    let localization_system = Localization::from_reader(file_fr)
        .expect(format!("{}::{} Could not create the localization system with the french translation", file!(), line!()).as_str());
    //warning
    //greeting
    //Attacked
    //quit
    //bye
    assert_eq!(localization_system.get("warning"), Some("Fais attention"));
    assert_eq!(localization_system.get("greeting"), Some("Bonjour"));
    assert_eq!(localization_system.get("Attacked"), Some("Je suis attaqué"));
    assert_eq!(localization_system.get("quit"), Some("Quitter le jeu"));
    assert_eq!(localization_system.get("bye"), Some("Au revoir"));
    assert!(localization_system.get("Healing").is_none());

    let localization_system = Localization::from_reader(file_en)
        .expect(format!("{}::{} Could not create the localization system with the english translation", file!(), line!()).as_str());
    assert_eq!(localization_system.get("warning"), Some("Be careful"));
    assert_eq!(localization_system.get("greeting"), Some("Hello"));
    assert_eq!(localization_system.get("Attacked"), Some("I am under attack"));
    assert_eq!(localization_system.get("quit"), Some("Quit the game"));
    assert_eq!(localization_system.get("bye"), Some("Good bye"));
    assert!(localization_system.get("Healing").is_none());

    let localization_system = Localization::from_reader(file_es)
        .expect(format!("{}::{} Could not create the localization system with the spanish translation", file!(), line!()).as_str());
    assert_eq!(localization_system.get("warning"), Some("Ten cuidado"));
    assert_eq!(localization_system.get("greeting"), Some("Hola"));
    assert_eq!(localization_system.get("Attacked"), Some("Estoy atacado"));
    assert_eq!(localization_system.get("quit"), Some("Salir del juego"));
    assert_eq!(localization_system.get("bye"), Some("Adios"));
    assert!(localization_system.get("Healing").is_none());
}