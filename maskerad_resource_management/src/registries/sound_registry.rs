// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use resources::sound_resource::SoundResource;
use std::io::{Read, Seek};

pub struct SoundRegistry<'a, R: 'a + Read + Seek>(HashMap<PathBuf, &'a SoundResource<R>>);

impl<'a, R: Read + Seek> Default for SoundRegistry<'a, R> {
    fn default() -> Self {
        debug!("Creating a default SoundRegistry.");
        SoundRegistry(HashMap::default())
    }
}

impl<'a, R: Read + Seek> SoundRegistry<'a, R> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the SoundRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&SoundResource<R>> {
        debug!("Trying to get a sound resource with path {}", path.as_ref().display());
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&SoundResource<R>> {
        debug!("Removing a sound resource with path {}", path.as_ref().display());
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, sound: &'a SoundResource<R>) -> Option<&SoundResource<R>> where
        I: Into<PathBuf>,
    {
        debug!("Inserting a sound resource into the SoundRegistry.");
        self.0.insert(path.into(),sound)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the SoundRegistry.");
        self.0.clear();
    }
}