// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use std::collections::HashMap;
use std::path::{PathBuf, Path};
use imagefmt::Image;

#[derive(Debug)]
pub struct TgaResource(Image<u8>);

impl From<Image<u8>> for TgaResource {
    fn from(image: Image<u8>) -> Self {
        TgaResource(image)
    }
}

impl AsRef<TgaResource> for TgaResource {
    fn as_ref(&self) -> &TgaResource {
        self
    }
}

impl AsRef<Image<u8>> for TgaResource {
    fn as_ref(&self) -> &Image<u8> {
        &self.0
    }
}

#[derive(Debug)]
pub struct TgaRegistry<'a>(HashMap<PathBuf, &'a TgaResource>);

impl<'a> Default for TgaRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default TgaRegistry.");
        TgaRegistry(HashMap::default())
    }
}

impl<'a> TgaRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the TgaRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&TgaResource> {
        debug!("Trying to get a reference to a tga image in the TgaRegistry.");
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&TgaResource> {
        debug!("Removing a tga image in the TgaRegistry.");
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, tga_res: &'a TgaResource) -> Option<&TgaResource> where
        I: Into<PathBuf>,
    {
        debug!("Inserting a tga image in the TgaRegistry.");
        self.0.insert(path.into(),tga_res)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the tga registry.");
        self.0.clear();
    }
}

