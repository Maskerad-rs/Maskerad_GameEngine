use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;

use systems::system_implementations::platforms::linux::filesystem::Filesystem;
use systems::system_implementations::platforms::PlatformType;

use std::path::{Path, PathBuf};
use std::sync::Arc;
//TODO: Maybe Mutex ?
//There's a multitude of implementations, according to the target platform.


//TODO: BUILDERS SHOULD READ CONFIG FILES
//TODO: Consume should return a GameResult, if an option hasn't been modified to a Some() (mean config file not complete)
pub struct FileSystemBuilder {
    number_of_thread: Option<usize>,
    root: Option<PathBuf>,
    read_only: Option<bool>,
    platform: Option<PlatformType>,
}

impl FileSystemBuilder {
    pub fn new() -> Self {
        FileSystemBuilder {
            number_of_thread: None,
            root: None,
            read_only: None,
            platform: None,
        }
    }

    pub fn for_the_platform(mut self, platform: PlatformType) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn with_number_of_thread(mut self, size: usize) -> Self {
        self.number_of_thread = Some(size);
        self
    }

    pub fn with_root(mut self, root: &Path) -> Self {
        self.root = Some(root.to_path_buf());
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }

    pub fn consume(&mut self) -> Arc<VFilesystem>  {
        let root = match self.root {
            Some(ref path) => path.clone(),
            None => PathBuf::from("root"),
        };

        let readonly = match self.read_only {
            Some(value) => value,
            None => false,
        };

        let number_of_thread = match self.number_of_thread {
            Some(number) => number,
            None => 4,
        };

        match self.platform {
            Some(ref platform) => {
                match platform {
                    &PlatformType::Linux => {
                        Arc::new(Filesystem::new(root.as_path(), readonly))
                    },
                }
            },
            None => {
                Arc::new(Filesystem::new(root.as_path(), readonly))
            }
        }
    }
}



#[cfg(test)]
mod filesystem_builder_test {
    use super::*;

    #[test]
    fn test() {
        let mut filesystem_builder = FileSystemBuilder::new()
            .with_number_of_thread(5)
            .with_read_only(true)
            .with_root(PathBuf::from("root_test").as_path());

        let new_file_system = filesystem_builder.consume().unwrap();
    }
}