// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;

use imagefmt::Error as ImageError;
use lewton::VorbisError as SoundError;
use gltf::Error as ModelError;

#[derive(Debug)]
pub enum ResourceError {
    ImageError(String, ImageError),
    SoundError(String, SoundError),
    ModelError(String, ModelError),
}

unsafe impl Send for ResourceError {}
unsafe impl Sync for ResourceError {}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResourceError::ImageError(ref desc, _) => {
                write!(f, "Image resource error: {}", desc)
            },
            &ResourceError::SoundError(ref desc, _) => {
                write!(f, "Sound resource error: {}", desc)
            },
            &ResourceError::ModelError(ref desc, _) => {
                write!(f, "Model resource error: {}", desc)
            },
        }
    }
}

impl Error for ResourceError {
    fn description(&self) -> &str {
        match self {
            &ResourceError::ImageError(_, _) => {
                "ImageError"
            },
            &ResourceError::SoundError(_, _) => {
                "SoundError"
            },
            &ResourceError::ModelError(_, _) => {
                "ModelError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ResourceError::ImageError(_, ref image_error) => {
                Some(image_error)
            },
            &ResourceError::SoundError(_, ref sound_error) => {
                Some(sound_error)
            },
            &ResourceError::ModelError(_, ref model_error) => {
                Some(model_error)
            },
        }
    }
}

pub type ResourceResult<T> = Result<T, ResourceError>;

impl From<ImageError> for ResourceError {
    fn from(error: ImageError) -> Self {
        ResourceError::ImageError(String::from("Error while loading an image."), error)
    }
}

impl From<SoundError> for ResourceError {
    fn from(error: SoundError) -> Self {
        ResourceError::SoundError(String::from("Error while loading a sound file."), error)
    }
}

impl From<ModelError> for ResourceError {
    fn from(error: ModelError) -> Self {
        ResourceError::ModelError(String::from("Erro while loading a model file."), error)
    }
}