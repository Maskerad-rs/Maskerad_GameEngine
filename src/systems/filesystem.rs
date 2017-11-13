use std::fs;
use std::path::{Path, PathBuf};

use core::engine_support_systems::system_management::filesystems::{VFilesystem, VMetadata, VFile, OpenOptions};
use core::engine_support_systems::system_management::system_types::{VSystem, SystemType};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};

//The Filesystem must:
//- Give access to files

//game name (root)
//logs
//
#[derive(Debug)]
pub struct Filesystem {
    root: PathBuf,
    readonly: bool,
}

pub struct Metadata(fs::Metadata);
impl VMetadata for Metadata {
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    fn is_file(&self) -> bool {
        self.0.is_file()
    }
    fn len(&self) -> u64 {
        self.0.len()
    }
}

impl VSystem for Filesystem {
    fn system_type() -> SystemType {
        SystemType::Filesystem
    }

    fn start_up(&mut self) -> GameResult<()> {

    }

    fn shut_down(&mut self) -> GameResult<()> {

    }
}

impl VFilesystem for Filesystem {
    fn open_with_options(&self, path: &Path, open_options: &OpenOptions) -> GameResult<Box<VFile>> {
        match open_options.open(path) {
            Ok(file) => file,
            Err(e) => GameError::FileSystemError(format!("Could not create file {} !", path), e),
        }
    }

    fn mkdir(&self, path: &Path) -> GameResult<()> {
        match fs::create_dir(path) {
            Ok(()) => Ok(()),
            Err(e) => GameError::FileSystemError(format!("Could not create directory {} !", path), e),
        }
    }

    fn rmrf(&self, path: &Path) -> GameResult<()> {
        match fs::remove_dir_all(Path) {
            Ok(()) => Ok(()),
            Err(e) => GameError::FileSystemError(format!("Could not delete the directory and its contents {} !", path), e),
        }
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn metadata(&self, path: &Path) -> GameResult<Box<VMetadata>> {

    }
}