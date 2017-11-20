use core::engine_support_systems::data_structures::threadpools::systems::filesystem::filesystem_threadpool_messages::FilesystemMessage;
use core::engine_support_systems::system_management::systems::filesystems::VFilesystem;

use std::path::Path;
use std::io::BufReader;
use std::io::Read;

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct FilesystemWorker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl FilesystemWorker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<FilesystemMessage>>>, filesystem: Arc<VFilesystem> ) -> Self {
        let thread = thread::spawn(move || {
            loop {
                //We use recv, not try_recv. recv always block the current thread if there's no data available.
                //However, if i'm not mistaken, recv isn't invoked in the main thread, so there's no big deal.
                //and try_recv returns an error if the buffer is empty. Definitely not adapted to a thread pool.
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    FilesystemMessage::RemoveFile(name) => {
                        filesystem.rm(Path::new(name.as_str())).unwrap();
                    },
                    FilesystemMessage::RemoveDirectory(name) => {
                        filesystem.rmrf(Path::new(name.as_str())).unwrap();
                    },
                    //A worker thread cannot join (he loops forever). However, we can pass Arc<Mutex<Types>> to our messages in order to fill them ?
                    FilesystemMessage::ReadFile(name, string_to_fill) => {
                        let file = filesystem.open(Path::new(name.as_str())).unwrap();
                        let mut buf_reader = BufReader::new(file);
                        buf_reader.read_to_string(&mut *string_to_fill.lock().unwrap()).unwrap();
                    },
                    FilesystemMessage::ReadDirectory(name, vec_to_fill) => {
                        //TODO: a string to fill ? maybe a vec<PathBuf> right ?
                        //let file_iterator = filesystem.read_dir(Path::new(name.as_str())).unwrap();
                        for path in filesystem.read_dir(Path::new(name.as_str())).unwrap() {
                            vec_to_fill.lock().unwrap().push(path.unwrap());
                        }
                    },
                    FilesystemMessage::CreateDirectory(name) => {
                        filesystem.mkdir(Path::new(name.as_str())).unwrap();
                    },
                    FilesystemMessage::AppendToFile(name, text) => {
                        //append only open the file in a certain state, you have to write after.
                        let mut file = filesystem.append(Path::new(name.as_str())).unwrap();
                        file.write_all(text.as_bytes()).unwrap();
                    },
                    FilesystemMessage::WriteToFile(name, text) => {
                        //same for create.
                        let mut file = filesystem.create(Path::new(name.as_str())).unwrap();
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