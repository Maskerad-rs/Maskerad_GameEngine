// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use lewton::inside_ogg::OggStreamReader;
use std::io::{Read, Seek};

pub struct OggResource<R: Read + Seek>(OggStreamReader<R>);

impl<R: Read + Seek> From<OggStreamReader<R>> for OggResource<R> {
    fn from(ogg: OggStreamReader<R>) -> Self {
        OggResource(ogg)
    }
}

impl<R: Read + Seek> AsRef<OggResource<R>> for OggResource<R> {
    fn as_ref(&self) -> &OggResource<R> {
        self
    }
}

impl<R: Read + Seek> AsRef<OggStreamReader<R>> for OggResource<R> {
    fn as_ref(&self) -> &OggStreamReader<R> {
        &self.0
    }
}

pub struct OggRegistry<'a, R: 'a + Read + Seek>(HashMap<PathBuf, &'a OggResource<R>>);

impl<'a, R: Read + Seek> Default for OggRegistry<'a, R> {
    fn default() -> Self {
        debug!("Creating a default OggRegistry.");
        OggRegistry(HashMap::default())
    }
}

impl<'a, R: Read + Seek> OggRegistry<'a, R> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the OggRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&OggResource<R>> {
        debug!("Trying to get an ogg resource with path {}", path.as_ref().display());
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&OggResource<R>> {
        debug!("Removing ogg resource with path {}", path.as_ref().display());
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, ogg_res: &'a OggResource<R>) -> Option<&OggResource<R>> where
        I: Into<PathBuf>,
    {
        debug!("Inserting an ogg resource into the OggRegistry.");
        self.0.insert(path.into(),ogg_res)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the ogg registry.");
        self.0.clear();
    }
}