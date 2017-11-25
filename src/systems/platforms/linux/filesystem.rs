

use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;
use std::io;

use std::env;

use rayon;

use core::engine_support_systems::system_management::systems::filesystems::{VFilesystem, VMetadata, VFile, OpenOptions};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};
use core::engine_support_systems::system_management::System;
use core::engine_support_systems::system_management::SystemType;
use core::engine_support_systems::system_management::PlatformType;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync;

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
    fn file_type(&self) -> fs::FileType {
        self.0.file_type()
    }
    fn is_read_only(&self) -> bool {
        self.0.permissions().readonly()
    }
}



pub struct Filesystem {
    root: PathBuf,
    //TODO: Should the filesystem contain 'conventional paths' ? (resource directory, log directory...).
}

impl System for Filesystem {
    fn system_type(&self) -> SystemType {
        SystemType::Filesystem
    }

    fn platform(&self) -> PlatformType {
        PlatformType::Linux
    }

    fn shut_down(&self) -> GameResult<()> {
        Ok(())
    }
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Filesystem root: {}>", self.root.display())
    }
}

impl Filesystem {

    //create the filesystem and the root directory (the current directory).
    //The working directory is changed to the root directory of a unix filesystem.
    pub fn new() -> GameResult<Filesystem> {
        match env::current_dir() {
            Ok(path) => {
                Ok(Filesystem {
                    root:  path.clone(),
                })
            },
            Err(error) => Err(GameError::IOError(format!("Could not create the filesystem !"), error))
        }
    }

    //Used to check the path given by the user.
    fn get_absolute(&self, path: &Path) -> GameResult<PathBuf> {
        let mut root_path = self.current_directory()?;
        root_path.push(path);
        Ok(root_path)
    }
}

impl VFilesystem for Filesystem {



    fn display_current_directory(&self) -> GameResult<()> {
        println!("{}", self.current_directory()?.display());
        Ok(())
    }

    fn current_directory(&self) -> GameResult<PathBuf> {
        env::current_dir().map(|path| path).map_err(GameError::from)
    }

    fn set_current_directory(&self, path: &Path) -> GameResult<()> {
        env::set_current_dir(path).map_err(GameError::from)
    }

    fn root_directory(&self) -> PathBuf {
        self.root.clone()
    }

    fn open_with_options(&self, path: &Path, open_options: &OpenOptions) -> GameResult<Box<VFile>> {
        let absolute_path = self.get_absolute(path)?;

        open_options
            .to_fs_openoptions()
            .open(absolute_path.clone().as_path())
            .map(|file| Box::new(file) as Box<VFile>).
            map_err(GameError::from)
    }

    fn mkdir(&self, path: &Path) -> GameResult<()> {
        let absolute_path = self.get_absolute(path)?;
        fs::DirBuilder::new().recursive(true).create(absolute_path.as_path()).map_err(GameError::from)
    }

    fn rm(&self, path: &Path) -> GameResult<()> {
        let absolute_path = self.get_absolute(path)?;
        if absolute_path.is_dir() {
            fs::remove_dir(path).map_err(GameError::from)
        } else {
            fs::remove_file(path).map_err(GameError::from)
        }
    }

    fn rmrf(&self, path: &Path) -> GameResult<()> {
        let absolute_path = self.get_absolute(path)?;
        if absolute_path.is_dir() {
            fs::remove_dir_all(path).map_err(GameError::from)
        } else {
            Err(GameError::FileSystemError(format!("({}) is not a directory !, use rm instead.", absolute_path.display())))
        }
    }

    fn exists(&self, path: &Path) -> bool {
        match self.get_absolute(path) {
            Ok(p) => p.exists(),
            _ => false,
        }
    }

    fn metadata(&self, path: &Path) -> GameResult<Box<VMetadata>> {
        let absolute_path = self.get_absolute(path)?;
        absolute_path.metadata().map(|m| {
            Box::new(Metadata(m)) as Box<VMetadata>
        }).map_err(GameError::from)
    }

    fn read_dir(&self, path: &Path) -> GameResult<Vec<fs::DirEntry>> {
        let absolute_path = self.get_absolute(path)?;
        let mut vec = Vec::new();

        if absolute_path.is_dir() {

             for entry in fs::read_dir(absolute_path.as_path())? {
                 let entry = entry?;
                 let path = entry.path();
                 if path.is_file() {
                     vec.push(entry);
                 }
             }
        } else {
            return Err(GameError::FileSystemError(format!("the path ({}) must be a directory !", absolute_path.display())));
        }

        Ok(vec)
    }
}


//TODO: test the physical filesystem
#[cfg(test)]
mod linux_filesystem_test {
    use super::*;
    use std::io::BufReader;
    use std::io::Read;

    #[test]
    fn filesystem_mkdir() {
        let filesystem = Filesystem::new().unwrap();
        let mut dir_test = filesystem.root_directory();
        dir_test.push(Path::new("dir_test"));
        filesystem.mkdir(dir_test.as_path()).unwrap();
        assert!(filesystem.exists(dir_test.as_path()));

        filesystem.create(Path::new("dir_test/file_test.txt")).expect("Couldn't create file").write_all(b"text_test\n").expect("Couldn't create file and add 'text test'");
        filesystem.append(Path::new("dir_test/file_test.txt")).expect("Couldn't append to file").write_all(b"text_append_test\n").expect("Couldn't append to file and add 'text_append-test'");
        let mut bufreader = BufReader::new(filesystem.open(Path::new("dir_test/file_test.txt")).expect("Couldn't read file with bufreader"));
        let mut content = String::new();
        bufreader.read_to_string(&mut content);
        let mut lines = content.lines();
        println!("{:?}", content);
        assert_eq!(lines.next(), Some("text_test"));
        assert_eq!(lines.next(), Some("text_append_test"));
        assert_eq!(lines.next(), None);

        let file_metadata = filesystem.metadata(Path::new("dir_test/file_test.txt")).expect("Couldn't get metadata");
        assert!(file_metadata.is_file());
        assert!(!file_metadata.is_dir());

        filesystem.create(Path::new("dir_test/file_test_rm.txt")).expect("Couldn't create file").write_all(b"test rm\n").expect("Coudln't create file and write test rm");
        filesystem.create(Path::new("dir_test/file_test_rm_2.txt")).expect("Couldn't create file").write_all(b"test rm 2\n").expect("Coudln't create file and write test rm 2");
        filesystem.rm(Path::new("dir_test/file_test_rm_2.txt")).expect("Couldn't delete the file : file_test_rm_2.txt");
        assert!(!filesystem.exists(Path::new("dir_test/file_test_rm_2.txt")));
        filesystem.rmrf(Path::new("dir_test")).expect("Couldn't delete dir");
        assert!(!filesystem.exists(Path::new("dir_test")));
    }


    #[test]
    fn filesystem_current_working_directory() {
        let filesystem = Filesystem::new().expect("Could not create FS");
        assert_eq!(filesystem.current_directory().expect("Couldn't get working directory"), filesystem.root_directory());
    }


    #[test]
    fn filesystem_read_dir() {
        let filesystem = Filesystem::new().expect("Couldn't create FS");
        let entries = filesystem.read_dir(Path::new("src")).unwrap();
        let mut iter = entries.iter();
        assert!(iter.next().is_some()); //lib.rs
        assert!(iter.next().is_none()); //nothing, not recursive
    }
}