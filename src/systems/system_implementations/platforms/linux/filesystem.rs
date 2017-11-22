

use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;

use std::env;

use core::engine_support_systems::system_management::systems::filesystems::{VFilesystem, VMetadata, VFile, OpenOptions, sanitize_path};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};
use core::engine_support_systems::system_management::System;
use core::engine_support_systems::system_management::SystemType;
use core::engine_support_systems::system_management::PlatformType;

use std::sync::Arc;

//The Filesystem must:
//- Give access to files

//game name (root)
//logs
//

//TODO: Partially rewrite the linux filesystem.
//TODO: stop using unwrap, handle those goddamn errors with our GameResult<>

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
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Filesystem root: {}>", self.root.display())
    }
}

impl Filesystem {

    //create the filesystem and the root directory (the current directory).
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
        root_path.push(self.root_directory());
        root_path.push(path);

        match sanitize_path(root_path.as_path()) {
            Some(path) => Ok(path),
            None => Err(GameError::FileSystemError(format!("Could not obtain an absolute path from the given path ({}) relative to the filesystem's root directory !", root_path.display())))
        }
    }
}

impl VFilesystem for Filesystem {

    fn shut_down(&self) -> GameResult<()> {
        Ok(())
    }

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
            fs::remove_file(path).map_err(GameError::from)
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

    //We return an Arc<Iterator<Item=GameResult<PathBuf>>>, not a box. Our filesystem threadpool, with its workers, ask the filesystem
    //to return an iterator of GameResult<PathBuf>. However, our workers are in other threads.
    //Walk a directory, only visiting files
    fn read_dir(&self, path: &Path) -> GameResult<Box<Iterator<Item = GameResult<PathBuf>>>> {
        let absolute_path = self.get_absolute(path)?;

        let itr = fs::read_dir(path)?
            .map(|entry| {
                let filename = entry.unwrap().file_name().into_string().unwrap();
                let mut pathbuf = PathBuf::from(path);
                pathbuf.push(filename);
                Ok(pathbuf)
            })
            .collect::<Vec<GameResult<PathBuf>>>()
            .into_iter();

        Ok(Box::new(itr))
    }
}


//TODO: test the physical filesystem
#[cfg(test)]
mod linux_filesystem_test {
    use super::*;
    use std::io::BufReader;
    use std::io::Read;

    #[test]
    fn test_path_filtering() {
        let p = Path::new("/foo");
        sanitize_path(p).unwrap();

        let p = Path::new("/foo/");
        sanitize_path(p).unwrap();

        let p = Path::new("/foo/bar.txt");
        sanitize_path(p).unwrap();

        let p = Path::new("/");
        sanitize_path(p).unwrap();

        let p = Path::new("../foo");
        assert!(sanitize_path(p).is_none());

        let p = Path::new("foo");
        assert!(sanitize_path(p).is_none());

        let p = Path::new("/foo/../../");
        assert!(sanitize_path(p).is_none());

        let p = Path::new("/foo/../bop");
        assert!(sanitize_path(p).is_none());

        let p = Path::new("/../bar");
        assert!(sanitize_path(p).is_none());

        let p = Path::new("");
        assert!(sanitize_path(p).is_none());
    }

    #[test]
    fn filesystem_mkdir() {
        let filesystem = Filesystem::new().unwrap();
        filesystem.mkdir(Path::new("/dir_test")).unwrap();
        let mut path = filesystem.root.clone();
        path.push(Path::new("/dir_test"));
        assert!(filesystem.exists(path.as_path()));
    }

    #[test]
    fn filesystem_open_then_write_then_append_then_read() {
        let filesystem = Filesystem::new().unwrap();
        filesystem.create(Path::new("/dir_test/file_test.txt")).expect("Couldn't create file").write_all(b"text_test\n").expect("Couldn't create file and add 'text test'");
        filesystem.append(Path::new("/dir_test/file_test.txt")).expect("Couldn't append to file").write_all(b"text_append_test\n").expect("Couldn't append to file and add 'text_append-test'");
        let mut bufreader = BufReader::new(filesystem.open(Path::new("/dir_test/file_test.txt")).expect("Couldn't read file with bufreader"));
        let mut content = String::new();
        bufreader.read_to_string(&mut content);

        let mut lines = content.lines();

        println!("{:?}", content);
        assert_eq!(lines.next(), Some("text_test"));
        assert_eq!(lines.next(), Some("text_append_test"));
        assert_eq!(lines.next(), None);
    }

    //TODO: Add tests.
}