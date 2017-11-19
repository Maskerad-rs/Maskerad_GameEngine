use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_worker_messages::FilesystemWorkerMessage;
use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_threadpool_messages::FilesystemMessage;

use std::path::Path;
use std::io::BufReader;
use std::io::Read;

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct FilesystemWorker {
    id: usize,
    thread: thread::JoinHandle<()>, //TODO: Maybe JoinHandle<GameResult<()>> ?
}

impl FilesystemWorker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<FilesystemWorkerMessage>>> ) -> Self {

        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap(); //TODO:recv or try_recv ??

                match job.0 {
                    FilesystemMessage::RemoveFile(name) => {
                        job.1.rm(Path::new(name.as_str())).unwrap();
                    },
                    FilesystemMessage::RemoveDirectory(name) => {
                        job.1.rmrf(Path::new(name.as_str())).unwrap();
                    },
                    FilesystemMessage::ReadFile(name, string_to_fill) => {
                        let file = job.1.open(Path::new(name.as_str())).unwrap();
                        let mut buf_reader = BufReader::new(file);
                        buf_reader.read_to_string(&mut *string_to_fill.lock().unwrap());
                    },
                    FilesystemMessage::ReadDirectory(name, string_to_fill) => {
                        //TODO: not sure if it's the right thing to do.
                        let file_iterator = job.1.read_dir(Path::new(name.as_str())).unwrap();
                        for file in file_iterator {

                        }
                    },
                    FilesystemMessage::CreateDirectory(name) => {
                        job.1.mkdir(Path::new(name.as_str())).unwrap();
                    },
                    FilesystemMessage::AppendToFile(name, text) => {
                        //append only open the file in a certain state, you have to write after.
                        let file = job.1.append(Path::new(name.as_str())).unwrap();
                        //TODO: write, or write_all ?
                        file.write_all(text.as_bytes()).unwrap();
                    },
                    FilesystemMessage::WriteToFile(name, text) => {
                        //same for create.
                        let file = job.1.create(Path::new(name.as_str())).unwrap();
                        //TODO: write, or write_all ?
                        file.write_all(text.as_bytes()).unwrap();
                    }
                }
            }
        });

        FilesystemWorker {
            id,
            thread,
        }
    }
}