use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Error as FileError;

#[derive(Debug)]
pub struct LogError {
    description: String,
    cause: FileError,
}

impl Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LogError: {}", self.description)
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        "LogError"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}

type KindredLogResult<T> = Result<T, LogError>; // See how GGEZ do.