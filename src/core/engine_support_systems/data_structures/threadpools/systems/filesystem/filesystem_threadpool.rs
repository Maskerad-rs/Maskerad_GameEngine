use core::engine_support_systems::error_handling::error::{GameResult, GameError};
use core::engine_support_systems::data_structures::ThreadPool;
use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_worker::FilesystemWorker;
use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;
use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_threadpool_messages::FilesystemMessage;

use std::fmt;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct FilesystemThreadPool {
    workers: Vec<FilesystemWorker>,
    sender: mpsc::Sender<FilesystemMessage>,
}

impl ThreadPool for FilesystemThreadPool {
    fn get_number_of_thread(&self) -> usize {
        self.workers.len()
    }
}

impl fmt::Debug for FilesystemThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filesystem threadpool: nb threads {}, sends {:?}", self.get_number_of_thread(), self.sender)
    }
}

impl FilesystemThreadPool {

    //TODO: A Function join(&self, worker_id: usize) { workers[worker_id].get_thread().join(); } ??

    pub fn execute(&self, message: FilesystemMessage) {
        self.sender.send(message).unwrap();
    }

    pub fn new(size: usize, filesystem: Arc<VFilesystem>) -> GameResult<FilesystemThreadPool> {
        if size <= 0 {
            Err(GameError::ThreadPoolError(String::from("The FilesystemThreadPool was initialized with a null or negative number of workers !")))
        } else {

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(FilesystemWorker::new(id, receiver.clone(), filesystem.clone()));
            }

            Ok(FilesystemThreadPool {
                workers,
                sender,
            })
        }
    }
}