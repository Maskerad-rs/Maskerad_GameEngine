// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Error as IOError;
use std::fmt;
use std::error::Error;
use toml::de::Error as TomlDeserError;
use toml::ser::Error as TomlSerError;

#[derive(Debug)]
pub enum EngineConfigError {
    IOError(String, IOError),
    TomlSerError(String, TomlSerError),
    TomlDeserError(String, TomlDeserError),
}

unsafe impl Send for EngineConfigError {}
unsafe impl Sync for EngineConfigError {}

impl fmt::Display for EngineConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EngineConfigError::IOError(ref desc, _) => {
                write!(f, "I/O error: {}", desc)
            },
            &EngineConfigError::TomlDeserError(ref desc, _) => {
                write!(f, "TOML deserialization error: {}", desc)
            },
            &EngineConfigError::TomlSerError(ref desc, _) => {
                write!(f, "TOML serialization error: {}", desc)
            },
        }
    }
}

impl Error for EngineConfigError {
    fn description(&self) -> &str {
        match self {
            &EngineConfigError::IOError(_, _) => {
                "I/O error"
            },
            &EngineConfigError::TomlSerError(_, _) => {
                "TomlSerError"
            },
            &EngineConfigError::TomlDeserError(_, _) => {
                "TomlDeserError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &EngineConfigError::IOError(_, ref io_error) => {
                Some(io_error)
            },
            &EngineConfigError::TomlSerError(_, ref ser_error) => {
                Some(ser_error)
            },
            &EngineConfigError::TomlDeserError(_, ref deser_error) => {
                Some(deser_error)
            },
        }
    }
}

pub type EngineConfigResult<T> = Result<T, EngineConfigError>;

impl From<IOError> for EngineConfigError {
    fn from(error: IOError) -> Self {
        EngineConfigError::IOError(format!("Error while doing I/O operations."), error)
    }
}

impl From<TomlDeserError> for EngineConfigError {
    fn from(error: TomlDeserError) -> Self {
        EngineConfigError::TomlDeserError(format!("Error while deserializing a TOML file."), error)
    }
}

impl From<TomlSerError> for EngineConfigError {
    fn from(error: TomlSerError) -> Self {
        EngineConfigError::TomlSerError(format!("Error while serializing a structure to a TOML file."), error)
    }
}
