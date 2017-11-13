use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;

use core::engine_support_systems::system_management::filesystems::{VFilesystem, VMetadata, VFile, OpenOptions};
use core::engine_support_systems::system_management::system_types::{VSystem, SystemType};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};

//The Filesystem must:
//- Give access to files

//game name (root)
//logs
//

pub struct Filesystem {
    root: PathBuf,
    readonly: bool,
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Filesystem root: {}>", self.root.display())
    }
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
    fn system_type(&self) -> SystemType {
        SystemType::Filesystem
    }

    fn shut_down(&mut self) -> GameResult<()> {

    }
}

impl Filesystem {
    fn get_absolute(&self, path: &Path) -> GameResult<PathBuf> {
        if let Some(safe_path) = self.sanitize_path(path) {
            let mut root_path = self.root.clone();
            root_path.push(safe_path);
            Ok(root_path)
        } else {
            Err(GameError::FileSystemError(format!("Path {:?} is not valid: must be an absolute path with no references to parent directories", path), None))
        }
    }
}

impl VFilesystem for Filesystem {
    fn open_with_options(&self, path: &Path, open_options: &OpenOptions) -> GameResult<Box<VFile>> {
        if self.readonly && (open_options.is_write() || open_options.is_create() || open_options.is_append() || open_options.is_truncate()) {
            return Err(GameError::FileSystemError(format!("Cannot alter file {:?} in root {:?}, filesystem read-only", path, self), None));
        }

        let absolute_path = self.get_absolute(path)?;
        open_options.to_fs_openoptions().open(absolute_path).map(|x| {
            Box::new(x)
        }).map_err(GameError::from)
    }

    fn mkdir(&self, path: &Path) -> GameResult<()> {
        match fs::create_dir(path) {
            Ok(()) => Ok(()),
            Err(e) => GameError::FileSystemError(format!("Could not create directory {} !", path), e),
        }
    }

    fn rmrf(&self, path: &Path) -> GameResult<()> {
        match fs::remove_dir_all(path) {
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