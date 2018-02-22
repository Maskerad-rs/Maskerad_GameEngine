// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter};
use filesystem::game_directories::{GameDirectories, RootDir};
use filesystem::filesystem_error::{FileSystemError, FileSystemResult};
use filesystem::open_options::OpenOptions;
use remove_dir_all;

//Open to read file
//Open to write to file
//Create file if it doesn't exist
//Append to file
//Access to metadata

/*FILESYSTEM.

A filesystem must provide the following functionalities :
- Manipulating file names and paths.
- open close read write append create files and directory.
- scan content of directory.
_____________________________________________________________
- asynchronous I/O (streaming music or textures...). -> just use Rayon directly.

TODO: Take a look at how mio handle async io with TCP. Or future stuff.
_____________________________________________________________
*/

#[derive(Debug)]
pub struct Filesystem {
    directories: GameDirectories,
}

impl Filesystem {
    pub fn new(game_name: &str, game_author: &str) -> FileSystemResult<Self> {
        debug!("Creating a new Filesystem with the game name {}, created by {}", game_name, game_author);
        let directories = GameDirectories::new(game_name, game_author)?;

        Ok(Filesystem {
            directories,
        })
    }

    pub fn get_absolute_path<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<PathBuf> {
        debug!("Getting the absolute path of {}", path.as_ref().display());
        fs::canonicalize(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    }

    //Open file at path with options
    fn open_with_options<P, O>(&self, path: P, open_options: O) -> FileSystemResult<File> where
        P: AsRef<Path>,
        O: AsRef<OpenOptions>,
    {
        trace!("Opening file at path {} with options {}", path.as_ref().display(), open_options.as_ref());
        open_options.as_ref()
            .to_fs_openoptions()
            .open(path.as_ref())
            .map_err(|io_error| FileSystemError::from(io_error))
    }

    //Open file at path to read
    pub fn open<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<BufReader<File>> {
        debug!("Opening file at path {}", path.as_ref().display());
        let buf = self.open_with_options(path.as_ref(), OpenOptions::new().set_read(true))?;
        Ok(BufReader::new(buf))
    }

    //Open file at path for writing, truncates if file already exist
    pub fn create<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<BufWriter<File>> {
        debug!("Creating/truncating file at path {}", path.as_ref().display());
        let buf = self.open_with_options(
            path.as_ref(),
            OpenOptions::new()
                .set_create(true)
                .set_write(true)
                .set_truncate(true),
        )?;
        Ok(BufWriter::new(buf))
    }

    //Open the file at path for appending, creating it if necessary
    pub fn append<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<BufWriter<File>> {
        debug!("Appending/Creating file at path {}", path.as_ref().display());
        let buf = self.open_with_options(
            path.as_ref(),
            OpenOptions::new()
                .set_create(true)
                .set_append(true)
                .set_write(true),
        )?;
        Ok(BufWriter::new(buf))
    }

    //create directory at path
    pub fn mkdir<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<()> {
        debug!("Creating directory at path {}", path.as_ref().display());
        fs::DirBuilder::new()
            .recursive(true)
            .create(path.as_ref())
            .map_err(|io_error| FileSystemError::from(io_error))
    }

    //remove a file
    pub fn rm<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<()> {
        if path.as_ref().is_dir() {
            debug!("Removing empty directory at path {}", path.as_ref().display());
            fs::remove_dir(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
        } else {
            debug!("Removing file at path: {}", path.as_ref().display());
            fs::remove_file(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
        }
    }

    //remove file or directory and all its contents
    pub fn rmrf<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<()> {
        debug!("Removing file/dir at path {}", path.as_ref().display());
        remove_dir_all::remove_dir_all(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    }

    //Retrieve all file entries in the given directory (recursive).
    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> FileSystemResult<fs::ReadDir> {
        debug!("Getting all entries in the directory at path {}", path.as_ref().display());
        fs::read_dir(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    }

    fn path(&self, root_dir: RootDir) -> FileSystemResult<PathBuf> {
        debug!("Getting the full path of the {}.", root_dir);
        match self.directories.get(&root_dir) {
            Some(pathbuf_ref) => {
                trace!("Found the path of the {}.", root_dir);
                Ok(pathbuf_ref.clone())
            },
            None => {
                error!("Could not find the path of the {} !", root_dir);
                Err(FileSystemError::GameDirectoryError(format!(
                    "The associated path for {:?} could not be found !",
                    root_dir
                )))
            },
        }
    }

    pub fn construct_path_from_root(
        &self,
        root_dir: RootDir,
        path: &str,
    ) -> FileSystemResult<PathBuf> {
        debug!("Creating the full path of {}, according to the {}", path, root_dir);
        let mut root_dir = self.path(root_dir)?;
        root_dir.push(path);
        Ok(root_dir)
    }
}

#[cfg(test)]
mod filesystem_test {
    use super::*;
    use std::io::Write;
    use filesystem::game_directories::{GameDirectories, RootDir};

    #[test]
    fn filesystem_io_operations() {
        let fs =
            Filesystem::new("test_filesystem_maskerad", "Malkaviel")
                .expect("Couldn't create FS");

        let current_dir_dir_test = fs
            .construct_path_from_root(RootDir::WorkingDirectory, "dir_test")
            .expect("Could not create current_dir_dir_test PathBuf");

        fs.mkdir(current_dir_dir_test.as_path())
            .expect("Could not create dir with current_dir_dir_test as path");
        assert!(current_dir_dir_test.exists());

        //user logs
        let user_log_dir_test = fs
            .construct_path_from_root(RootDir::EngineLogRoot, "log_dir_test")
            .expect("Could not create user_log_dir_test");
        fs.mkdir(user_log_dir_test.as_path())
            .expect("Could not create dir with user_log_dir_test as path");
        assert!(user_log_dir_test.exists());

        let file_test = fs
            .construct_path_from_root(RootDir::EngineLogRoot, "log_dir_test/file_test.txt")
            .expect("Could not create file_test.txt");
        let mut log_dir_bufwriter =
            fs.create(file_test.as_path()).expect("Could not create log_dir_test/file_test.txt");

        log_dir_bufwriter.write_all(b"text_test\n").unwrap();

        /*
        let async_dir = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "async_dir")
            .expect("Could not create async_dir");
        mkdir(async_dir.as_path()).expect("Could not create dir with async_dir as path");
        assert!(async_dir.exists());

        //test async functionalities.
        let thread_pool = Configuration::new()
            .build()
            .expect("Could not create the thread pool.");

        let async_log_dir_test = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt")
            .expect("Could not create async_log_dir_test");
        {
            let mut log_bufwriter =
                create(async_log_dir_test.as_path()).expect("Could not create the bufwriter");


            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_1\n").unwrap()
            });

            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_2\n").unwrap()
            });

            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_3\n").unwrap()
            });

        } //bufwriter dropped here, all the write calls will be executed.


        let mut bufreader_async =
            open(async_log_dir_test.as_path()).expect("Could not create bufreader");
        let mut content = String::new();


        thread_pool.install(|| {
            bufreader_async.read_to_string(&mut content).unwrap()
        });

        let mut lines = content.lines();
        assert_eq!(lines.next(), Some("test_async_text_1"));
        assert_eq!(lines.next(), Some("test_async_text_2"));
        assert_eq!(lines.next(), Some("test_async_text_3"));
        assert_eq!(lines.next(), None);

        //Metadata
        let file_metadata = async_log_dir_test
            .metadata()
            .expect("Couldn't get metadata");
        assert!(file_metadata.is_file());
        assert!(!file_metadata.is_dir());
        assert!(file_metadata.len() > 0);

        //remove
        rm(async_log_dir_test.as_path())
            .expect("Couldn't delete the file : async_dir/async_log_dir_test.txt");
        assert!(!async_log_dir_test.exists());
        rmrf(current_dir_dir_test.as_path()).expect("Couldn't delete dir");
        assert!(!current_dir_dir_test.exists());
        */
    }

    #[test]
    fn filesystem_read_dir() {
        let fs =
            Filesystem::new("test_filesystem_blacksmith", "Malkaviel")
                .expect("Couldn't create GameDirs");
        let src_dir = fs
            .construct_path_from_root(RootDir::WorkingDirectory, "src")
            .unwrap();
        let mut entries = fs.read_dir(src_dir).unwrap();
        assert!(entries.next().is_some());
    }
}
