use std::fmt;
use std::error;

#[derive(Debug)]
pub enum TError{
    IO(std::io::Error),
    Serde(serde_json::Error),
    NonExistentKey,
    FaultyCommandInLog,
}

impl fmt::Display for TError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TError::IO(e) =>
                write!(f, "Error in IO: {}", e),
            TError::Serde(e) =>
                write!(f, "Error in bincode crate: {}", e),
            TError::NonExistentKey => 
                write!(f, "Key not found"),
            TError::FaultyCommandInLog =>
                write!(f, "Error reading log file"),
        }
    }
}

impl error::Error for TError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TError::IO(ref e) => Some(e),
            TError::Serde(ref e) => Some(e),
            TError::NonExistentKey => None,
            TError::FaultyCommandInLog => None,
        }
    }
}

impl From<std::io::Error> for TError {
    fn from(err : std::io::Error) -> TError {
        TError::IO(err)
    }
}

impl From<serde_json::Error> for TError {
    fn from(err : serde_json::Error) -> TError {
        TError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, TError>;
