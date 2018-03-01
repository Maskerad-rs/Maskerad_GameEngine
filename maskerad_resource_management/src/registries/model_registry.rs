// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use resources::model_resource::ModelResource;

#[derive(Debug)]
pub struct ModelRegistry<'a>(HashMap<PathBuf, &'a ModelResource>);

impl<'a> Default for ModelRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default ModelRegistry.");
        ModelRegistry(HashMap::default())
    }
}

impl<'a> ModelRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the ModelRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&ModelResource> {
        debug!("Trying to get a model resource with path {}.", path.as_ref().display());
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&ModelResource> {
        debug!("Removing a model resource with path {}.", path.as_ref().display());
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, model: &'a ModelResource) -> Option<&ModelResource> where
        I: Into<PathBuf>,
    {
        debug!("Inserting a model resource.");
        self.0.insert(path.into(),model)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the ModelRegistry.");
        self.0.clear();
    }
}