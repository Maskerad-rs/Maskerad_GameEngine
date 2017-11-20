use std::sync::Arc;
use std::sync::Mutex;

use std::path::PathBuf;

pub enum FilesystemMessage {
    ReadFile(String, Arc<Mutex<String>>),
    WriteToFile(String, String),
    AppendToFile(String, String),
    CreateDirectory(String),
    RemoveFile(String),
    RemoveDirectory(String),
    ReadDirectory(String, Arc<Mutex<Vec<PathBuf>>>),
}