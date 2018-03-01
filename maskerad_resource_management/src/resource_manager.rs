// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use registries::image_registry::ImageRegistry;
use registries::model_registry::ModelRegistry;
use registries::sound_registry::SoundRegistry;

use resources::sound_resource::SoundResource;
use resources::model_resource::ModelResource;
use resources::image_resource::{ImageResource, ColorFormat};

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use maskerad_core::allocators::stacks::MemStack;

use std::path::{PathBuf, Path};
use std::io::{Read, Seek};
use std::cell::{RefCell, Cell, Ref, RefMut};

pub struct ResourceManager<'a, R: 'a + Read + Seek> {
    double_ended_allocator: (MemStack, MemStack),
    image_resources: RefCell<ImageRegistry<'a>>,
    model_resources: RefCell<ModelRegistry<'a>>,
    sound_resources: RefCell<SoundRegistry<'a, R>>,
    marker_global_resource: Cell<usize>,
    marker_global_resource_copy: Cell<usize>,
}

impl<'a, R: 'a + Read + Seek> ResourceManager<'a, R> {
    pub fn with_capacity(capacity: usize, capacity_copy: usize) -> Self {
        ResourceManager {
            double_ended_allocator: (MemStack::with_capacity(capacity/2, capacity_copy/2), MemStack::with_capacity(capacity/2, capacity_copy/2)),
            image_resources: RefCell::new(ImageRegistry::new()),
            model_resources: RefCell::new(ModelRegistry::new()),
            sound_resources: RefCell::new(SoundRegistry::new()),
            marker_global_resource: Cell::new(0),
            marker_global_resource_copy: Cell::new(0),
        }
    }

    pub fn set_marker_global_resources(&self, marker: usize) {
        debug!("Setting the marker to the end of the global resources.");
        self.marker_global_resource.set(marker);
    }

    pub fn marker_global_resources(&self) -> usize {
        debug!("Getting the marker to the end of the global resources.");
        self.marker_global_resource.get()
    }

    pub fn set_marker_global_resources_copy(&self, marker: usize) {
        debug!("Setting the marker to the end of the global resources (copy).");
        self.marker_global_resource_copy .set(marker);
    }

    pub fn marker_global_resources_copy(&self) -> usize {
        debug!("Getting the marker to the end of the global resources (copy).");
        self.marker_global_resource_copy.get()
    }

    pub fn image_resources(&'a self) -> Ref<ImageRegistry> {
        self.image_resources.borrow()
    }

    pub fn image_resources_mut(&'a self) -> RefMut<ImageRegistry> {
        self.image_resources.borrow_mut()
    }

    pub fn model_resources(&'a self) -> Ref<ModelRegistry> {
        self.model_resources.borrow()
    }

    pub fn model_resources_mut(&'a self) -> RefMut<ModelRegistry> {
        self.model_resources.borrow_mut()
    }

    pub fn sound_resources(&'a self) -> Ref<SoundRegistry<R>> {
        self.sound_resources.borrow()
    }

    pub fn sound_resources_mut(&'a self) -> RefMut<SoundRegistry<R>> {
        self.sound_resources.borrow_mut()
    }

    pub fn load_image<P>(&'a self, path: P, reader: &mut R, requested_format: ColorFormat) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading image data with path {} in resource manager.", path.as_ref().display());
        //Move the image in an allocator
        let image_ref = self.double_ended_allocator.0.allocate( || {
            ImageResource::from_reader(reader, requested_format).expect("Could not create an image resource from reader")
        })?;
        //Add in registry.
        self.image_resources.borrow_mut().insert(path.as_ref(), image_ref);
        Ok(())
    }

    pub fn load_model<P>(&'a self, path: P, reader: R) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading model data with path {} in resource manager.", path.as_ref().display());
        //create gltf.
        let model_ref = self.double_ended_allocator.0.allocate(|| {
            ModelResource::from_reader( reader).expect("Could not create a model resource from reader")
        })?;
        //Add in registry.
        self.model_resources.borrow_mut().insert(path.as_ref(), model_ref);
        Ok(())
    }

    pub fn load_sound<P>(&'a self, path: P, reader: R) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>,
    {
        debug!("Loading sound data with path {} in resource manager.", path.as_ref().display());
        //create ogg.
        let sound_ref = self.double_ended_allocator.0.allocate(|| {
            SoundResource::from_reader(reader).expect("Could not create a sound resource from reader")
        })?;
        //Add in registry
        self.sound_resources.borrow_mut().insert(path.as_ref(), sound_ref);
        Ok(())
    }

    pub fn clear(&self) {
        debug!("unloading resources from the resource manager.");
        //The game has been closed if the global resources must be unloaded. Clear everything.
        self.model_resources.borrow_mut().clear();
        self.sound_resources.borrow_mut().clear();
        self.image_resources.borrow_mut().clear();
        self.double_ended_allocator.0.reset();
        self.double_ended_allocator.0.reset_copy();
        self.double_ended_allocator.1.reset();
        self.double_ended_allocator.1.reset_copy();
    }

    //TODO: needed ?
    pub fn unload_temporary_data(&self) {
        debug!("Unloading temporary data.");
        self.double_ended_allocator.1.reset();
        self.double_ended_allocator.1.reset_copy();
    }
}

#[cfg(test)]
mod resource_manager_test {
    use super::*;
    use std::io::BufReader;
    use std::fs::File;
    #[test]
    fn resource_manager_creation() {
        let resource_manager: ResourceManager<BufReader<File>> = ResourceManager::with_capacity(100, 100);
        assert!(resource_manager.model_resources().is_empty());
        assert!(resource_manager.sound_resources().is_empty());
        assert!(resource_manager.image_resources().is_empty());
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

