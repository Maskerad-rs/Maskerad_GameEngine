// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use std::collections::HashMap;
use std::path::{PathBuf, Path};
use resources::image_resource::ImageResource;

#[derive(Debug)]
pub struct ImageRegistry<'a>(HashMap<PathBuf, &'a ImageResource>);

impl<'a> Default for ImageRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default ImageRegistry.");
        ImageRegistry(HashMap::default())
    }
}

impl<'a> ImageRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the ImageRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&ImageResource> {
        debug!("Trying to get a reference to an image in the ImageRegistry.");
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&ImageResource> {
        debug!("Removing an image in the ImageRegistry.");
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, image: &'a ImageResource) -> Option<&ImageResource> where
        I: Into<PathBuf>,
    {
        debug!("Inserting an image in the ImageRegistry.");
        self.0.insert(path.into(),image)
    }

    pub fn clear(&mut self) {
        debug!("Clearing the ImageRegistry.");
        self.0.clear();
    }
}

