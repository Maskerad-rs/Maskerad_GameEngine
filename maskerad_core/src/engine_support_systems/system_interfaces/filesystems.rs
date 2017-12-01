// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;
use std::path::{PathBuf, Path, Component};
use std::io::{Read, Seek, Write};
use std::fmt;

use std::sync::Arc;
use std::sync::Mutex;


use game_infos::GameInfos;

use engine_support_systems::error_handling::error::GameResult;
use engine_support_systems::system_interfaces::System;

//Enum used to specify the 'root' directory from where to write/delete/open dir/files
#[derive(Debug, Copy, Clone)]
pub enum RootDir {
    WorkingDirectory,
    UserDataRoot,
    UserConfigRoot,
    UserEngineConfigurationRoot,
    UserLogRoot,
    UserSaveRoot,
}


//We create a VFile trait to pave the way to different type of files.
pub trait VFile: Read + Seek + Write + fmt::Debug {}
//TODO: Think about the different types of files (StreamableTexture ? MusicFile ? ShaderFile ? LogFile ?)

impl<T: Read + Seek + Write + fmt::Debug> VFile for T {}

//Rust provides metadata about files. We provide a VMetadata trait to pave the way to different type of metadata about different files.
//Metadata, with the cross-platform fs module, only
pub trait VMetadata {
    //Is it a directory ?
    fn is_dir(&self) -> bool;
    //Is it a file ?
    fn is_file(&self) -> bool;
    //The length of the thing.
    fn len(&self) -> u64;
    //Is the file read only ?
    fn is_read_only(&self) -> bool;
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

    // Open for writing
    pub fn set_write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    // Create the file if it does not exist yet
    pub fn set_create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    // Append at the end of the file
    pub fn set_append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }

    // Truncate the file to 0 bytes after opening
    pub fn set_truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
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

/*FILESYSTEM.

A filesystem must provide the following functionalities :
- Manipulating file names and paths.
- open close read write append create files and directory.
- does a file or directory exists ?.
- get metadata about files.
- scan content of directory.
- asynchronous I/O (streaming music or textures...).
*/
pub trait VFilesystem : System {

    fn application_info(&self) -> &GameInfos;

    //Open file at path with options
    fn open_with_options(&self, root_dir: RootDir, path: &str, open_options: &OpenOptions) -> GameResult<Box<VFile>>;

    //Open file at path to read
    fn open(&self, root_dir: RootDir, path: &str) -> GameResult<Box<VFile>> {
        self.open_with_options(root_dir, path, OpenOptions::new().set_read(true))
    }
    //Open file at path for writing, truncates if file already exist
    fn create(&self, root_dir: RootDir, path: &str) -> GameResult<Box<VFile>> {
        self.open_with_options( root_dir, path, OpenOptions::new().set_create(true).set_write(true).set_truncate(true))
    }
    //Open the file at path for appending, creating it if necessary
    fn append(&self, root_dir: RootDir, path: &str) -> GameResult<Box<VFile>> {
        self.open_with_options(root_dir, path, OpenOptions::new().set_create(true).set_append(true).set_write(true))
    }
    //create directory at path
    fn mkdir(&self, root_dir: RootDir, path: &str) -> GameResult<()>;
    //remove a file
    fn rm(&self, root_dir: RootDir, path: &str) -> GameResult<()>;
    //remove file or directory and all its contents
    fn rmrf(&self, root_dir: RootDir, path: &str) -> GameResult<()>;
    //Check if file exists
    fn exists(&self, root_dir: RootDir, path: &str) -> bool;

    //Get file's metadata
    //Arc because FS threadpool asks FS to give him. But workers in others threads.
    fn metadata(&self, root_dir: RootDir, path: &str) -> GameResult<Box<VMetadata>>;

    //Retrieve all file entries in the given directory (recursive).
    //Arc because FS threadpool asks FS to give him. But workers in others threads.
    fn read_dir(&self, root_dir: RootDir, path: &str) -> GameResult<fs::ReadDir>;
}