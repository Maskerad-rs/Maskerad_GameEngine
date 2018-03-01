// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use gltf::{Gltf, Glb};
use std::io::Read;

use resources::resource_errors::ResourceResult;

#[derive(Debug)]
pub struct ModelResource(Gltf);

impl From<Gltf> for ModelResource {
    fn from(model: Gltf) -> Self {
        ModelResource(model)
    }
}

impl ModelResource {
    pub fn from_glb<'a, G: 'a>(glb: G) -> ResourceResult<ModelResource> where
        G: AsRef<Glb<'a>>
    {
        let gltf = Gltf::from_glb(glb.as_ref())?.validate_completely()?;
        Ok(ModelResource(gltf))
    }

    pub fn from_reader<R: Read>(reader: R) -> ResourceResult<ModelResource> {
        let gltf = Gltf::from_reader(reader)?.validate_completely()?;
        Ok(ModelResource(gltf))
    }
}