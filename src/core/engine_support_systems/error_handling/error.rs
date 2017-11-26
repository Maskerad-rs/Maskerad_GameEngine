//A convenient error wrapper,
//largely inspired by GGEZ's error and Result type : Concise and totally do the job.

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Error as FileError;
use std::sync;
use std::fs::DirEntry;

use app_dirs;

//The GameError enum implement the Error trait, bound to the Debug + Display traits
//Some enum's struct contain only a description, while other contain a lower-level error,
//which is meant to be passed to the cause() function from the Error trait.

//If a GameError type have multiple causes of failure, create another enum specialized in this system
//for example, if LogError can have multiple cause of failure, create the enum LogErrorType.
#[derive(Debug)]
pub enum GameError {
    IOError(String, FileError),
    FileSystemError(String),
    UnknownError(String),
    ThreadPoolError(String),
    ApplicationDirectoriesError(String, app_dirs::AppDirsError),
}

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GameError::IOError(ref description, _) => write!(f, "IO error: {}", description),
            &GameError::FileSystemError(ref description) => write!(f, "File system error: {}", description),
            &GameError::UnknownError(ref description) => write!(f, "Unknown error: {}", description),
            &GameError::ThreadPoolError(ref description) => write!(f, "ThreadPool error: {}", description),
            &GameError::ApplicationDirectoriesError(ref description, _) => write!(f, "Application directories error: {}", description),
        }
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        match self {
            &GameError::IOError(_, _) => "LogError",
            &GameError::FileSystemError(_) => "FileSystemError",
            &GameError::UnknownError(_) => "UnknownError",
            &GameError::ThreadPoolError(_) => "ThreadPoolError",
            &GameError::ApplicationDirectoriesError(_, _) => "ApplicationDirectoriesError",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            &GameError::IOError(_, ref file_error) => {
                Some(file_error)
            },
            &GameError::FileSystemError(_) => {
                None
            },
            &GameError::UnknownError(_) => {
                None
            },
            &GameError::ThreadPoolError(_) => {
                None
            },
            &GameError::ApplicationDirectoriesError(_, ref app_dir_error) => {
                Some(app_dir_error)
            },
        }
    }
}

pub type GameResult<T> = Result<T, GameError>;

impl From<FileError> for GameError {
    fn from(error: FileError) -> Self {
        GameError::IOError(format!("Error while dealing with file"), error)
    }
}

impl From<app_dirs::AppDirsError> for GameError {
    fn from(error: app_dirs::AppDirsError) -> Self {
        GameError::ApplicationDirectoriesError(format!("Error while dealing with application directories"), error)
    }
}