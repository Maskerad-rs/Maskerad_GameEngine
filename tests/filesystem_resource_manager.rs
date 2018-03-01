extern crate maskerad_game_engine;

use maskerad_game_engine::core::filesystem::filesystem::Filesystem;
use maskerad_game_engine::core::filesystem::game_directories::RootDir;

use maskerad_game_engine::resource_management::resource_manager::ResourceManager;
use maskerad_game_engine::resource_management::resources::{model_resource::ModelResource, image_resource::{ImageResource, ColorFormat}, sound_resource::SoundResource};

#[test]
fn resource_manager_load_unload_get_resource() {
    //Filesystem, StackAlloc, ResourceManager.
    let fs = Filesystem::new("test_resource_man", "Malkaviel")
        .expect(format!("{}::{} Could not create fs.", file!(), line!()).as_str());
    let mut resource_man = ResourceManager::with_capacity(10000000, 10000000); //10 mb

    //Load image
    let image_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "test_resources/images/Untitled.tga")
        .expect(format!("{}::{} Could not create tga path.", file!(), line!()).as_str());
    let mut image_reader = Filesystem::open(image_path.as_path())
        .expect(format!("{}::{} Could no create image reader.", file!(), line!()).as_str());
    resource_man.load_image(image_path.as_path(), &mut image_reader, ColorFormat::Auto)
        .expect(format!("{}::{} Could not load image in resource manager", file!(), line!()).as_str());
    assert!(!resource_man.image_resources().is_empty());
    assert!(resource_man.image_resources().get(image_path.as_path()).is_some());

    //Load gltf
    let model_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "test_resources/gltf/untitled.gltf")
        .expect(format!("{}::{} Could not create the model_path", file!(), line!()).as_str());

    let model_reader = Filesystem::open(model_path.as_path())
        .expect(format!("{}::{} Could not create a reader to read the model's file.", file!(), line!()).as_str());
    resource_man.load_model(model_path.as_path(), model_reader)
        .expect(format!("{}::{} Could not put the model in the resource manager.", file!(), line!()).as_str());
    assert!(!resource_man.model_resources().is_empty());
    assert!(resource_man.model_resources().get(model_path.as_path()).is_some());

    //Load ogg
    let sound_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "test_resources/ogg/untitled.ogg")
        .expect(format!("{}::{} Could not create sound path.", file!(), line!()).as_str());
    let sound_reader = Filesystem::open(sound_path.as_path())
        .expect(format!("{}::{} Could not create sound reader", file!(), line!()).as_str());
    resource_man.load_sound(sound_path.as_path(), sound_reader)
        .expect(format!("{}::{} Could not load sound resource in the resource manager", file!(), line!()).as_str());
    assert!(!resource_man.sound_resources().is_empty());
    assert!(resource_man.sound_resources().get(sound_path.as_path()).is_some());

    //unload
    resource_man.clear();
    assert!(resource_man.sound_resources().is_empty());
    assert!(resource_man.model_resources().is_empty());
    assert!(resource_man.image_resources().is_empty());
}