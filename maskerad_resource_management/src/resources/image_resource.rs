// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use imagefmt::{Image, read_from, read, read_info, read_info_from, ColFmt, Info};
use std::io::{Read, Seek};
use std::path::Path;

use resources::resource_errors::{ResourceError, ResourceResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorFormat {
    Auto,
    Y,
    YA,
    AY,
    RGB,
    RGBA,
    BGR,
    BGRA,
    ARGB,
    ABGR,
}

impl Into<ColFmt> for ColorFormat {
    fn into(self) -> ColFmt {
        match self {
            ColorFormat::Auto => ColFmt::Auto,
            ColorFormat::Y => ColFmt::Y,
            ColorFormat::YA => ColFmt::YA,
            ColorFormat::AY => ColFmt::AY,
            ColorFormat::RGB => ColFmt::RGB,
            ColorFormat::RGBA => ColFmt::RGBA,
            ColorFormat::BGR => ColFmt::BGR,
            ColorFormat::BGRA => ColFmt::BGRA,
            ColorFormat::ARGB => ColFmt::ARGB,
            ColorFormat::ABGR => ColFmt::ABGR,
        }
    }
}

#[derive(Debug)]
pub struct ImageResource(Image<u8>);

impl From<Image<u8>> for ImageResource {
    fn from(image: Image<u8>) -> Self {
        ImageResource(image)
    }
}

impl ImageResource {
    pub fn from_path<P, C>(path: P, requested_format: C) -> ResourceResult<ImageResource> where
        P: AsRef<Path>,
        C: Into<ColFmt>
    {
        let img = read(path.as_ref(), requested_format.into())?;
        Ok(ImageResource(img))
    }

    pub fn from_reader<R: Read + Seek, C>(reader: &mut R, requested_format: C) -> ResourceResult<ImageResource> where
        C: Into<ColFmt>
    {
        let img = read_from(reader, requested_format.into())?;
        Ok(ImageResource(img))
    }

    pub fn infos_from_path<P>(path: P) -> ResourceResult<Info> where
        P: AsRef<Path>
    {
        read_info(path.as_ref()).map_err(|image_error| {
            ResourceError::from(image_error)
        })
    }

    pub fn infos_from_reader<R: Read + Seek>(reader: &mut R) -> ResourceResult<Info> {
        read_info_from(reader).map_err(|image_error| {
            ResourceError::from(image_error)
        })
    }
}

