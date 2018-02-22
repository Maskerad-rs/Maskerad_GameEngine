// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::env::VarError;

#[derive(Debug)]
pub enum FileSystemError {
    GameDirectoryError(String),
    CreationError(String),
    IOError(String, IOError),
    EnvironmentError(String, VarError),
    ExtensionError(String),
}

unsafe impl Send for FileSystemError {}
unsafe impl Sync for FileSystemError {}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FileSystemError::GameDirectoryError(ref description) => {
                write!(f, "Game directory error: {}", description)
            }
            &FileSystemError::CreationError(ref description) => {
                write!(f, "Creation error: {}", description)
            }
            &FileSystemError::EnvironmentError(ref description, _) => {
                write!(f, "Environment variable error: {}", description)
            }
            &FileSystemError::IOError(ref description, _) => {
                write!(f, "I/O error: {}", description)
            }
            &FileSystemError::ExtensionError(ref description) => {
                write!(f, "file extension error: {}", description)
            }
        }
    }
}

impl Error for FileSystemError {
    fn description(&self) -> &str {
        match self {
            &FileSystemError::GameDirectoryError(_) => "GameDirectoryError",
            &FileSystemError::CreationError(_) => "CreationError",
            &FileSystemError::EnvironmentError(_, _) => "EnvironmentError",
            &FileSystemError::IOError(_, _) => "IOError",
            &FileSystemError::ExtensionError(_) => "ExtensionError",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &FileSystemError::GameDirectoryError(_) => None,
            &FileSystemError::CreationError(_) => None,
            &FileSystemError::IOError(_, ref cause) => Some(cause),
            &FileSystemError::EnvironmentError(_, ref cause) => Some(cause),
            &FileSystemError::ExtensionError(_) => None,
        }
    }
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;

impl From<IOError> for FileSystemError {
    fn from(error: IOError) -> Self {
        FileSystemError::IOError(format!("Error while doing I/O operations"), error)
    }
}

impl From<VarError> for FileSystemError {
    fn from(error: VarError) -> Self {
        FileSystemError::EnvironmentError(
            format!("Error while dealing with environment variable"),
            error,
        )
    }
}
