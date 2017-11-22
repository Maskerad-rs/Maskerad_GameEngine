use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;
use core::engine_support_systems::system_management::PlatformType;

use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_threadpool::FilesystemThreadPool;

use core::engine_support_systems::error_handling::error::{GameError, GameResult};

use systems::system_implementations::platforms::linux;

use std::path::{Path, PathBuf};
use std::sync::Arc;
//TODO: Maybe Mutex ?
//There's a multitude of implementations, according to the target platform.


//TODO: BUILDERS SHOULD READ CONFIG FILES
pub struct FileSystemBuilder {
    number_of_thread: Option<usize>, //For the threadpool
    root: Option<PathBuf>, //for the filesystem creation
    platform: Option<PlatformType>, //which type of filesystem ?
}

impl FileSystemBuilder {
    pub fn new() -> Self {
        FileSystemBuilder {
            number_of_thread: None,
            root: None,
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

    pub fn consume(&mut self) -> GameResult<(FilesystemThreadPool, Arc<VFilesystem>)> {
        let root = match self.root {
            Some(ref path) => path.clone(),
            None => {
                return Err(GameError::FileSystemError(format!("The 'root' value was not specified in the filesystem builder !")))
            },
        };

        let number_of_thread = match self.number_of_thread {
            Some(number) => number,
            None => {
                return Err(GameError::FileSystemError(format!("The 'number of thread' was not specified in the filesystem builder !")))
            },
        };

        match self.platform {
            Some(ref platform) => {
                match platform {
                    &PlatformType::Linux => {
                        let filesystem = Arc::new(linux::filesystem::Filesystem::new()?);
                        let threadpool = FilesystemThreadPool::new(number_of_thread, filesystem.clone())?;
                        Ok((threadpool, filesystem))
                    },
                }
            },
            None => {
                Err(GameError::FileSystemError(format!("The 'platform' value was not specified in the filesystem builder !")))
            }
        }
    }
}



#[cfg(test)]
mod filesystem_builder_test {
    use super::*;
    use core::engine_support_systems::data_structures::ThreadPool;
    #[test]
    fn filesystem_builder_builds_for_different_platforms() {
        let mut filesystem_builder = FileSystemBuilder::new()
            .for_the_platform(PlatformType::Linux)
            .with_number_of_thread(5)
            .with_root(Path::new("/root_test"));

        let (threadpool, filesystem) = filesystem_builder.consume().unwrap();
        assert_eq!(threadpool.number_of_thread(), 5);
        assert_eq!(filesystem.platform(), PlatformType::Linux);
    }
}