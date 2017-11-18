use core::engine_support_systems::error_handling::error::{GameResult, GameError};

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
//Nightly only
use std::boxed::FnBox;




pub type FilesystemJob = Box<FnBox() + Send + 'static>;

pub struct FilesystemWorker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl FilesystemWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<FilesystemJob>>> ) -> Self {

        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap(); //TODO:recv or try_recv ??

                //execute the job
                job.call_box(());
            }
        });

        FilesystemWorker {
            id,
            thread,
        }
    }
}

pub struct FilesystemThreadPool {
    workers: Vec<FilesystemWorker>,
    sender: mpsc::Sender<FilesystemJob>,
}

impl FilesystemThreadPool {
    pub fn new(size: usize) -> GameResult<FilesystemThreadPool> {
        if size <= 0 {
            Err(GameError::ThreadPoolError(String::from("The FilesystemThreadPool was initialized with a null or negative number of workers !")))
        } else {

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(FilesystemWorker::new(id, receiver.clone()));
            }
            Ok(FilesystemThreadPool {
                workers,
                sender,
            })
        }
    }

    pub fn get_number_of_thread(&self) -> usize {
        self.workers.len()
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}