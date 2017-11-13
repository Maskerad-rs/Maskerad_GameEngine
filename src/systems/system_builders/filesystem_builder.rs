use core::engine_support_systems::system_management::system_types::{VSystem, VSystemBuilder, SystemType};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};
use systems::filesystem::Filesystem;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FilesystemBuilder {
    root: PathBuf,
    readonly: bool,
}

impl FilesystemBuilder {
    pub fn new(root: &Path, readonly: bool) -> Self {
        FilesystemBuilder {
            root,
            readonly,
        }
    }
}

impl VSystemBuilder for FilesystemBuilder {
    fn start_up(&self) -> GameResult<Box<VSystem>> {
        Ok(Box::new(Filesystem {
            root: self.root,
            readonly: self.readonly,
        }))
    }
}