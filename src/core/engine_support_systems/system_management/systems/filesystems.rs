use std::fs;
use std::path::{PathBuf, Path, Component};
use std::io::{Read, Seek, Write};
use std::fmt;

use std::sync::Arc;

use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::System;


pub trait VFile: Read + Seek + Write + fmt::Debug {}
impl<T: Read + Seek + Write + fmt::Debug> VFile for T {}

pub trait VMetadata {
    //Is it a directory ?
    fn is_dir(&self) -> bool;
    //Is it a file ?
    fn is_file(&self) -> bool;
    //The length of the thing.
    fn len(&self) -> u64;
}

// We need our own version of this structure because the one in
// std annoyingly doesn't let you get data out of it.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    append: bool,
    truncate: bool,
}

impl OpenOptions {
    // Create a new instance
    pub fn new() -> OpenOptions {
        Default::default()
    }

    // Open for reading
    pub fn set_read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    pub fn is_read(&self) -> bool {
        self.read
    }

    // Open for writing
    pub fn set_write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    pub fn is_write(&self) -> bool {
        self.write
    }

    // Create the file if it does not exist yet
    pub fn set_create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    pub fn is_create(&self) -> bool {
        self.create
    }

    // Append at the end of the file
    pub fn set_append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }

    pub fn is_append(&self) -> bool {
        self.append
    }

    // Truncate the file to 0 bytes after opening
    pub fn set_truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
    }

    pub fn is_truncate(&self) -> bool {
        self.truncate
    }

    pub fn to_fs_openoptions(&self) -> fs::OpenOptions {
        let mut opt = fs::OpenOptions::new();
        opt.read(self.read)
            .write(self.write)
            .create(self.create)
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create);
        opt
    }
}

//Open to read file
//Open to write to file
//Create file if it doesn't exist
//Append to file
//Access to metadata
pub trait VFilesystem : System {

    //TODO: SHOULD BE IN A SPECIAL TRAIT
    fn shut_down(&self) -> GameResult<()>;
    //

    //Open file at path with options
    fn open_with_options(&self, path: &Path, open_options: &OpenOptions) -> GameResult<Box<VFile>>;

    //Open file at path to read
    fn open(&self, path: &Path) -> GameResult<Box<VFile>> {
        self.open_with_options(path, OpenOptions::new().set_read(true))
    }
    //Open file at path for writing, truncates if file already exist
    fn create(&self, path: &Path) -> GameResult<Box<VFile>> {
        self.open_with_options(path, OpenOptions::new().set_create(true).set_write(true).set_truncate(true))
    }
    //Open the file at path for appending, creating it if necessary
    fn append(&self, path: &Path) -> GameResult<Box<VFile>> {
        self.open_with_options(path, OpenOptions::new().set_create(true).set_append(true).set_write(true))
    }
    //create directory at path
    fn mkdir(&self, path: &Path) -> GameResult<()>;
    //remove a file
    fn rm(&self, path: &Path) -> GameResult<()>;
    //remove file or directory and all its contents
    fn rmrf(&self, path: &Path) -> GameResult<()>;
    //Check if file exists
    fn exists(&self, path: &Path) -> bool;

    //Get file's metadata
    //Arc because FS threadpool asks FS to give him. But workers in others threads.
    fn metadata(&self, path: &Path) -> GameResult<Box<VMetadata>>;

    //Retrieve all file and directory entries in the given directory.
    //Arc because FS threadpool asks FS to give him. But workers in others threads.
    fn read_dir(&self, path: &Path) -> GameResult<Box<Iterator<Item = GameResult<PathBuf>>>>;
    //Retrieve the actual location of the filesystem root, if available
    fn to_path_buf(&self) -> Option<PathBuf>;
    //Takes an absolute path and returns either a sanitized relative version of it, or None
    //if there's something bad in it
    fn sanitize_path(&self, path: &Path) -> Option<PathBuf> {
        let mut path_components = path.components();
        match path_components.next() {
            Some(Component::RootDir) => (),
            _ => return None,
        }

        fn is_normal_component(comp: Component) -> Option<&str> {
            match comp {
                Component::Normal(s) => s.to_str(),
                _ => None,
            }
        }

        let mut sanitized_path = PathBuf::new();
        for component in path_components {
            if let Some(s) = is_normal_component(component) {
                sanitized_path.push(s)
            } else {
                return None;
            }
        }

        Some(sanitized_path)
    }
}