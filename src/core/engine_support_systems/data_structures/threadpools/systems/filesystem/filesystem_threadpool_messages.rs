use std::sync::Arc;
use std::sync::Mutex;

use std::path::PathBuf;
use std::fs;

pub enum FilesystemMessage {
    ReadFile(PathBuf, Arc<Mutex<String>>),
    WriteToFile(PathBuf, String),
    AppendToFile(PathBuf, String),
    CreateDirectory(PathBuf),
    RemoveFile(PathBuf),
    RemoveDirectory(PathBuf),
}