use std::sync::Arc;
use std::sync::Mutex;


pub enum FilesystemMessage {
    ReadFile(String, Arc<Mutex<String>>),
    WriteToFile(String, String),
    AppendToFile(String, String),
    CreateDirectory(String),
    RemoveFile(String),
    RemoveDirectory(String),
    ReadDirectory(String, Arc<Mutex<String>>),
}