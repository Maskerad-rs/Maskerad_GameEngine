// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::{PathBuf, Path};
use gltf::Gltf;
use lewton::inside_ogg::OggStreamReader;
use imagefmt::tga;
use imagefmt::ColFmt;
use maskerad_data_parser::level_description::LevelDescription;
use resources::resources_registry::ResourceRegistry;
//use resources::refcount_registry::RefCountRegistry;
use resources::ogg_registry::OggResource;
use resources::gltf_registry::GltfResource;
use resources::tga_registry::TgaResource;

use resources::resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use maskerad_memory_allocators::StackAllocator;
use maskerad_filesystem::filesystem::Filesystem;

use std::cell::{RefCell, Ref};
use std::io::BufReader;
use std::fs::File;
//TODO: We must get rid of the filesystem dependency injection, and rework the shit out of this goddamn resource manager.

pub struct ResourceManager<'a> {
    //A resources registry
    //An allocators registry
    double_ended_allocator: (StackAllocator, StackAllocator),
    global_resource_registry: RefCell<ResourceRegistry<'a>>,
    level_resource_registry: RefCell<ResourceRegistry<'a>>,
    marker_global_resource: usize,
    marker_global_resource_copy: usize,
}

impl<'a> ResourceManager<'a> {
    pub fn with_capacity(capacity: usize, capacity_copy: usize) -> Self {
        ResourceManager {
            double_ended_allocator: (StackAllocator::with_capacity(capacity/2, capacity_copy/2), StackAllocator::with_capacity(capacity/2, capacity_copy/2)),
            global_resource_registry: RefCell::new(ResourceRegistry::new()),
            level_resource_registry: RefCell::new(ResourceRegistry::new()),
            marker_global_resource: 0,
            marker_global_resource_copy: 0,
        }
    }

    pub fn set_marker_global_resources(&mut self, marker: usize) {
        debug!("Setting the marker to the end of the global resources.");
        self.marker_global_resource = marker;
    }

    pub fn marker_global_resources(&self) -> usize {
        debug!("Getting the marker to the end of the global resources.");
        self.marker_global_resource
    }

    pub fn set_marker_global_resources_copy(&mut self, marker: usize) {
        debug!("Setting the marker to the end of the global resources (copy).");
        self.marker_global_resource_copy = marker;
    }

    pub fn marker_global_resources_copy(&self) -> usize {
        debug!("Getting the marker to the end of the global resources (copy).");
        self.marker_global_resource_copy
    }

    pub fn level_resource_registry(&self) -> Ref<ResourceRegistry> {
        debug!("Borrowing an immutable reference to the level resource registry.");
        self.level_resource_registry.borrow()
    }

    pub fn global_resource_registry(&self) -> Ref<ResourceRegistry> {
        debug!("Borrowing an immutable reference to the global resource registry.");
        self.global_resource_registry.borrow()
    }

    //First step.
    fn read_needed_resources<I>(&self, level: I) -> Vec<String> where
        I: AsRef<LevelDescription>,
    {
        debug!("Reading all the resource needed for a level.");
        let mut vec: Vec<String> = Vec::new();

        //TODO: mesh
        for gameobject_builder in level.as_ref().slice() {
            if let Some(mesh_path) = gameobject_builder.get_mesh_resource() {
                if !vec.contains(&mesh_path) {
                    vec.push(mesh_path);
                }
            }

            //TODO: other resources
        }

        vec
    }

    fn load_tga<P>(&'a self, path: P, reader: &mut BufReader<File>) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading tga data with path {} in resource manager.", path.as_ref().display());
        //create tga.
        let tga_data = self.double_ended_allocator.0.alloc(|| {
            TgaResource::from(tga::read(reader, ColFmt::Auto).unwrap())
        })?;
        //Add in registry.
        self.level_resource_registry.borrow_mut().add_tga(path.as_ref(), tga_data);
        Ok(())
    }

    fn load_gltf<P>(&'a self, path: P, reader: &mut BufReader<File>) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading gltf data with path {} in resource manager.", path.as_ref().display());
        //create gltf.
        let gltf_data = self.double_ended_allocator.0.alloc(|| {
            GltfResource::from(Gltf::from_reader(reader).unwrap().validate_completely().unwrap())
        })?;
        //Add in registry.
        self.level_resource_registry.borrow_mut().add_gltf(path.as_ref(), gltf_data);
        Ok(())
    }

    fn load_ogg<P>(&'a self, path: P, reader: BufReader<File>) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading ogg data with path {} in resource manager.", path.as_ref().display());
        //create ogg.
        let ogg_data = self.double_ended_allocator.0.alloc(|| {
            OggResource::from(OggStreamReader::new(reader).unwrap())
        })?;
        //Add in registry
        self.level_resource_registry.borrow_mut().add_ogg(path.as_ref(), ogg_data);
        Ok(())
    }

    fn load_global_resources(&self) {
        unimplemented!()
    }

    fn load_global_resource<P>(&self, path: P, filesystem: &Filesystem) where
        P: AsRef<Path> + Into<PathBuf>,
    {
        unimplemented!()
    }

    fn clear(&self) {
        debug!("unloading global resources from the resource manager.");
        //The game has been closed if the global resources must be unloaded. Clear everything.
        self.level_resource_registry.borrow_mut().clear();
        self.global_resource_registry.borrow_mut().clear();
        self.double_ended_allocator.0.reset();
        self.double_ended_allocator.0.reset_copy();
        self.double_ended_allocator.1.reset();
        self.double_ended_allocator.1.reset_copy();
    }

    fn unload_level_resources(&self)
    {
        debug!("Unloading level resources from the resource manager.");
        self.level_resource_registry.borrow_mut().clear();
        self.double_ended_allocator.0.reset_to_marker(self.marker_global_resources());
        self.double_ended_allocator.0.reset_to_marker_copy(self.marker_global_resources_copy());
    }

    //TODO: needed ?
    fn unload_temporary_data(&self) {
        debug!("Unloading temporary data.");
        self.double_ended_allocator.1.reset();
        self.double_ended_allocator.1.reset_copy();
    }

    pub fn load_level_resources<L>(&'a self, level_description: L, filesystem: &Filesystem) -> ResourceManagerResult<()> where
        L: AsRef<LevelDescription>,
    {
        debug!("Reading all the resources needed by a level to load/unload all the resources.");
        /*
        When loading level:
        1 - Read all assets to load.
        2 - roll back to marker, just after the global resources.
        3 - Load all assets.
        */

        let needed_resources = self.read_needed_resources(level_description.as_ref());
        self.unload_level_resources();
        self.unload_temporary_data();
        for resource_str in needed_resources {
            let path: &Path = resource_str.as_ref();
            let mut reader = filesystem.open(path)?;

            match path.extension() {
                Some(osstr_ext) => {
                    match osstr_ext.to_str() {
                        Some(str_ext) => {
                            match str_ext {
                                "ogg" => {
                                    self.load_ogg(path, reader)?;
                                },
                                "tga" => {
                                    self.load_tga(path, &mut reader)?;
                                },
                                "gltf" => {
                                    self.load_gltf(path, &mut reader)?;
                                },
                                _ => {
                                    return Err(ResourceManagerError::ResourceError(format!("The data at path {} cannot be loaded by the engine !", path.display())));
                                }
                            }
                        },
                        None => {
                            return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid unicode !", path.display())));
                        }
                    }
                },
                None => {
                    return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid !", path.display())));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod resource_manager_test {
    use super::*;
    use maskerad_filesystem::game_directories::RootDir;
    #[test]
    fn resource_manager_creation() {
        let resource_manager = ResourceManager::with_capacity(100, 100);
        assert!(resource_manager.level_resource_registry.borrow().is_tga_empty());
        assert!(resource_manager.level_resource_registry.borrow().is_ogg_empty());
        assert!(resource_manager.level_resource_registry.borrow().is_gltf_empty());
        assert!(resource_manager.global_resource_registry.borrow().is_tga_empty());
        assert!(resource_manager.global_resource_registry.borrow().is_ogg_empty());
        assert!(resource_manager.global_resource_registry.borrow().is_gltf_empty());
    }

    #[test]
    fn resource_manager_load_unload_get_resource() {
        //Filesystem, StackAlloc, ResourceManager.
        let fs = Filesystem::new("test_resource_man", "Malkaviel").expect("Could not create fs.");
        let resource_man = ResourceManager::with_capacity(10000000, 10000000); //10 mb

        //Load tga
        let tga_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "tga_resource/Untitled.tga").expect("Could not create tga path.");
        let mut tga_reader = fs.open(tga_path.as_path()).expect("Could not creater tga reader.");
        resource_man.load_tga(tga_path.as_path(), &mut tga_reader).expect("Could not load tga image.");
        assert!(!resource_man.level_resource_registry.borrow().is_tga_empty());
        assert!(resource_man.level_resource_registry().get_tga(tga_path.as_path()).is_ok());

        //Load gltf
        let gltf_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "gltf_resource/untitled.gltf").expect("Could not create gltf path.");
        let mut gltf_reader = fs.open(gltf_path.as_path()).expect("Could not create gltf reader.");
        resource_man.load_gltf(gltf_path.as_path(), &mut gltf_reader).expect("Could not load gltf data.");
        assert!(!resource_man.level_resource_registry.borrow().is_gltf_empty());
        assert!(resource_man.level_resource_registry().get_gltf(gltf_path.as_path()).is_ok());

        //Load ogg
        let ogg_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "ogg_resource/untitled.ogg").expect("Could not create ogg path.");
        let mut ogg_reader = fs.open(ogg_path.as_path()).expect("Could not create ogg reader.");
        resource_man.load_ogg(ogg_path.as_path(), ogg_reader).expect("Could not load ogg data.");
        assert!(!resource_man.level_resource_registry.borrow().is_ogg_empty());
        assert!(resource_man.level_resource_registry().get_ogg(ogg_path.as_path()).is_ok());
        //unload
        resource_man.clear();
        assert!(resource_man.level_resource_registry.borrow().is_tga_empty());
        assert!(resource_man.level_resource_registry.borrow().is_ogg_empty());
        assert!(resource_man.level_resource_registry.borrow().is_gltf_empty());
        assert!(resource_man.global_resource_registry.borrow().is_tga_empty());
        assert!(resource_man.global_resource_registry.borrow().is_ogg_empty());
        assert!(resource_man.global_resource_registry.borrow().is_gltf_empty());

        //Load level.
        let level_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "toml_resource/level2.toml").expect("Could not create level path.");
        let mut level_reader = fs.open(level_path.as_path()).expect("Could not create level reader.");
        let level_desc = LevelDescription::load_from_toml(&mut level_reader).expect("Could not create level description.");
        resource_man.load_level_resources(&level_desc, &fs).expect("Could not load all level resources");
        assert!(resource_man.level_resource_registry.borrow().is_tga_empty());
        assert!(resource_man.level_resource_registry.borrow().is_ogg_empty());
        assert!(!resource_man.level_resource_registry.borrow().is_gltf_empty());
        assert!(resource_man.global_resource_registry.borrow().is_tga_empty());
        assert!(resource_man.global_resource_registry.borrow().is_ogg_empty());
        assert!(resource_man.global_resource_registry.borrow().is_gltf_empty());

        resource_man.clear();
    }

    #[test]
    fn resource_manager_load_unload_asynchronously_resource() {

    }

    #[test]
    fn resource_manager_post_process_resource() {
        //fine_tuning of the resource after it has been loaded
    }

    #[test]
    fn resource_manager_composite_resource_and_referential_integrity() {
        //Composite resource -> Model has mesh, anims, skeletons...
        //Referential integrity -> Model has a mesh, which has a skeletons and anims. Skeleton must be loaded before anims...
    }

    #[test]
    fn resource_manager_package_resources_in_one_big_file() {
        //Optional
    }
}