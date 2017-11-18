use systems::system_implementations::filesystem::Filesystem;
use core::engine_support_systems::data_structures::threadpools::filesystem_threadpool::FilesystemThreadPool;
use core::engine_support_systems::error_handling::error::{GameResult, GameError};

use std::path::{Path, PathBuf};



pub struct FileSystemBuilder {
    number_of_thread: Option<usize>,
    root: Option<PathBuf>,
    read_only: Option<bool>,
}

impl FileSystemBuilder {
    pub fn new() -> Self {
        FileSystemBuilder {
            number_of_thread: None,
            root: None,
            read_only: None,
        }
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

    pub fn consume(&mut self) -> GameResult<Filesystem> {
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

        let thread_pool = FilesystemThreadPool::new(number_of_thread)?;

        Ok(Filesystem::new(root.as_path(), readonly, thread_pool))
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