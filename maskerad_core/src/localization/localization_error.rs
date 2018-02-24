// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use serde_json::Error as JSONError;


#[derive(Debug)]
pub enum LocalizationError {
    IOError(String, IOError),
    JSONError(String, JSONError),
}

unsafe impl Send for LocalizationError {}
unsafe impl Sync for LocalizationError {}

impl fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LocalizationError::IOError(ref desc, _) => {
                write!(f, "I/O error: {}", desc)
            },
            &LocalizationError::JSONError(ref desc, _) => {
                write!(f, "JSON error: {}", desc)
            },
        }
    }
}

impl Error for LocalizationError {
    fn description(&self) -> &str {
        match self {
            &LocalizationError::IOError(_, _) => {
                "I/O error"
            },
            &LocalizationError::JSONError(_, _) => {
                "JSON error"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LocalizationError::IOError(_, ref io_error) => {
                Some(io_error)
            },
            &LocalizationError::JSONError(_, ref json_error) => {
                Some(json_error)
            },
        }
    }
}

pub type LocalizationResult<T> = Result<T, LocalizationError>;

impl From<IOError> for LocalizationError {
    fn from(error: IOError) -> Self {
        LocalizationError::IOError(format!("Error while doing filesystem I/O operations"), error)
    }
}

impl From<JSONError> for LocalizationError {
    fn from(error: JSONError) -> Self {
        LocalizationError::JSONError(format!("Error while serializing/deserializing a Rust structure/JSON file."), error)
    }
}