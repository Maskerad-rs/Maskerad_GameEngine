// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.



use resources::gltf_registry::{GltfRegistry, GltfResource};
use std::path::{Path, PathBuf};
use resources::resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use std::io::BufReader;
use std::fs::File;
use resources::ogg_registry::{OggRegistry, OggResource};
use resources::tga_registry::{TgaRegistry, TgaResource};
use std::collections::hash_map::Iter;

pub struct ResourceRegistry<'a> {
    gltf_registry: GltfRegistry<'a>,
    ogg_registry: OggRegistry<'a, BufReader<File>>,
    tga_registry: TgaRegistry<'a>,
}

impl<'a> Default for ResourceRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default ResourceRegistry.");
        ResourceRegistry {
            gltf_registry: GltfRegistry::default(),
            ogg_registry: OggRegistry::default(),
            tga_registry: TgaRegistry::default(),
        }
    }
}

impl<'a> ResourceRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    //____________________GLTF____________________________
    pub fn get_gltf<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<&&GltfResource> {
        debug!("Trying to get a gltf resource with path {}.", path.as_ref().display());
        match self.gltf_registry.get(path.as_ref()) {
            Some(gltf) => {
                trace!("The gltf resource has been found.");
                Ok(gltf)
            },
            None => {
                error!("The gltf resource could not be found.");
                Err(ResourceManagerError::ResourceError(format!("Could not find the gltf data at path {} in the gltf registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_gltf<I>(&mut self, path: I, gltf_resource: &'a GltfResource) -> Option<&GltfResource> where
        I: Into<PathBuf>,
    {
        debug!("Adding a gltf resource.");
        self.gltf_registry.insert(path, gltf_resource)
    }

    pub fn remove_gltf<I: AsRef<Path>>(&mut self, path: I) {
        debug!("Removing a gltf resource with path {}.", path.as_ref().display());
        self.gltf_registry.remove(path);
    }

    pub fn has_gltf<I: AsRef<Path>>(&self, path: I) -> bool {
        debug!("Checking if the ResourceManager has a gltf resource with path {}.", path.as_ref().display());
        self.gltf_registry.get(path).is_some()
    }

    pub fn is_gltf_empty(&self) -> bool {
        debug!("Checking if the ResourceManager is empty of gltf resources.");
        self.gltf_registry.is_empty()
    }


    //_________________________OGG______________________
    pub fn get_ogg<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<&&OggResource<BufReader<File>>> {
        debug!("Trying to get a ogg resource with path {}.", path.as_ref().display());
        match self.ogg_registry.get(path.as_ref()) {
            Some(ogg) => {
                trace!("The ogg resource has been found.");
                Ok(ogg)
            },
            None => {
                error!("The ogg resource could not be found.");
                Err(ResourceManagerError::ResourceError(format!("Could not find the ogg data at path {} in the ogg registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_ogg<I>(&mut self, path: I, ogg_resource: &'a OggResource<BufReader<File>>) -> Option<&OggResource<BufReader<File>>> where
        I: Into<PathBuf>,
    {
        debug!("Adding an ogg resource.");
        self.ogg_registry.insert(path, ogg_resource)
    }

    pub fn remove_ogg<I: AsRef<Path>>(&mut self, path: I) {
        debug!("Removing an ogg resource with path {}.", path.as_ref().display());
        self.ogg_registry.remove(path);
    }

    pub fn has_ogg<I: AsRef<Path>>(&self, path: I) -> bool {
        debug!("Checking if the ResourceManager has an ogg resource with path {}.", path.as_ref().display());
        self.ogg_registry.get(path).is_some()
    }

    pub fn is_ogg_empty(&self) -> bool {
        debug!("Checking if the ResourceManager is empty of ogg resources.");
        self.ogg_registry.is_empty()
    }

    //__________________________TGA_____________________
    pub fn get_tga<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<&&TgaResource> {
        debug!("Trying to get a tga resource with path {}.", path.as_ref().display());
        match self.tga_registry.get(path.as_ref()) {
            Some(tga) => {
                trace!("The tga resource has been found.");
                Ok(tga)
            },
            None => {
                error!("The tga resource could not be found.");
                Err(ResourceManagerError::ResourceError(format!("Could not find the tga data at path {} in the tga registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_tga<I>(&mut self, path: I, tga_resource: &'a TgaResource) -> Option<&TgaResource> where
        I: Into<PathBuf>,
    {
        debug!("Adding a tga resource.");
        self.tga_registry.insert(path, tga_resource)
    }

    pub fn remove_tga<I: AsRef<Path>>(&mut self, path: I) {
        debug!("Removing a tga resource with path {}.", path.as_ref().display());
        self.tga_registry.remove(path);
    }

    pub fn has_tga<I: AsRef<Path>>(&self, path: I) -> bool {
        debug!("Checking of the ResourceManager has a tga resource with path {}.", path.as_ref().display());
        self.tga_registry.get(path).is_some()
    }

    pub fn is_tga_empty(&self) -> bool {
        debug!("Checking if the ResourceManager is empty of tga resources.");
        self.tga_registry.is_empty()
    }

    pub fn clear(&mut self) {
        debug!("Clearing the resource registry.");
        self.tga_registry.clear();
        self.ogg_registry.clear();
        self.gltf_registry.clear();
    }
}