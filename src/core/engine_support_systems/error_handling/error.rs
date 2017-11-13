//A convenient error wrapper,
//largely inspired by GGEZ's error and Result type : Concise and totally do the job.

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Error as FileError;

//The GameError enum implement the Error trait, bound to the Debug + Display traits
//Some enum's struct contain only a description, while other contain a lower-level error,
//which is meant to be passed to the cause() function from the Error trait.

//If a GameError type have multiple causes of failure, create another enum specialized in this system
//for example, if LogError can have multiple cause of failure, create the enum LogErrorType.
#[derive(Debug)]
pub enum GameError {
    LogError(String, Option<FileError>),
    FileSystemError(String, FileError),
}

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GameError::LogError(ref description, ref file_error) => write!(f, "Log error: {}", description),
            &GameError::FileSystemError(ref description) => write!(f, "File system error: {}", description),

        }
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        match self {
            &GameError::LogError(_, _) => "LogError",
            &GameError::FileSystemError(_, _) => "FileSystemError",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            &GameError::LogError(ref description, ref file_error) => {
                match file_error {
                    &Some(error) => Some(&error),
                    &None => None,
                }
            },
            &GameError::FileSystemError(ref description, ref file_error) => {
                Some(&file_error)
            },


        }
    }
}

pub type GameResult<T> = Result<T, GameError>;