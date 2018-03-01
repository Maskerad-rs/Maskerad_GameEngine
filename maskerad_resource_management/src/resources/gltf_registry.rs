// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use gltf::Gltf;

#[derive(Debug)]
pub struct GltfResource(Gltf);

impl From<Gltf> for GltfResource {
    fn from(gltf: Gltf) -> Self {
        GltfResource(gltf)
    }
}

impl AsRef<GltfResource> for GltfResource {
    fn as_ref(&self) -> &GltfResource {
        self
    }
}

impl AsRef<Gltf> for GltfResource {
    fn as_ref(&self) -> &Gltf {
        &self.0
    }
}

#[derive(Debug)]
pub struct GltfRegistry<'a>(HashMap<PathBuf, &'a GltfResource>);

impl<'a> Default for GltfRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default GltfRegistry.");
        GltfRegistry(HashMap::default())
    }
}

impl<'a> GltfRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the GltfRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&GltfResource> {
        debug!("Trying to get a gltf resource with path {}.", path.as_ref().display());
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&GltfResource> {
        debug!("Removing a gltf resource with path {}.", path.as_ref().display());
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, gltf_res: &'a GltfResource) -> Option<&GltfResource> where
        I: Into<PathBuf>,
    {
        debug!("Inserting a gltf resource.");
        self.0.insert(path.into(),gltf_res)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the gltf registry.");
        self.0.clear();
    }
}