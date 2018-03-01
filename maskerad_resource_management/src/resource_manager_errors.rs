// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;
use resources::resource_errors::ResourceError;
use maskerad_core::filesystem::filesystem_error::FileSystemError;
use maskerad_core::allocators::errors::AllocationError;


#[derive(Debug)]
pub enum ResourceManagerError {
    FilesystemError(String, FileSystemError),
    ResourceError(String, ResourceError),
    AllocationError(String, AllocationError),
}

unsafe impl Send for ResourceManagerError {}
unsafe impl Sync for ResourceManagerError {}

impl fmt::Display for ResourceManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResourceManagerError::FilesystemError(ref description, _) => {
                write!(f, "Filesystem error: {}", description)
            },
            &ResourceManagerError::ResourceError(ref description, _) => {
                write!(f, "Resource error: {}", description)
            },
            &ResourceManagerError::AllocationError(ref description, _) => {
                write!(f, "Allocation error: {}", description)
            },
        }
    }
}

impl Error for ResourceManagerError {
    fn description(&self) -> &str {
        match self {
            &ResourceManagerError::FilesystemError(_, _) => {
                "ResourceNotFound"
            },
            &ResourceManagerError::ResourceError(_, _) => {
                "ResourceError"
            },
            &ResourceManagerError::AllocationError(_, _) => {
                "AllocationError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ResourceManagerError::FilesystemError(_, ref file_system_error) => {
                Some(file_system_error)
            },
            &ResourceManagerError::ResourceError(_, ref resource_error) => {
                Some(resource_error)
            },
            &ResourceManagerError::AllocationError(_, ref alloc_error) => {
                Some(alloc_error)
            },
        }
    }
}

pub type ResourceManagerResult<T> = Result<T, ResourceManagerError>;

impl From<FileSystemError> for ResourceManagerError {
    fn from(error: FileSystemError) -> Self {
        ResourceManagerError::FilesystemError(format!("Error while using the filesystem."), error)
    }
}

impl From<ResourceError> for ResourceManagerError {
    fn from(error: ResourceError) -> Self {
        ResourceManagerError::ResourceError(format!("Error while loading a resource."), error)
    }
}

impl From<AllocationError> for ResourceManagerError {
    fn from(error: AllocationError) -> Self {
        ResourceManagerError::AllocationError(format!("Error while allocating something in an allocator."), error)
    }
}