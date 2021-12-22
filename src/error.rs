use std::fmt;
use std::error;

#[derive(Debug)]
pub enum TError{
    IO(std::io::Error),
    Bincode(bincode::Error),
    NonExistentKey,
    FaultyCommandInLog,
}

impl fmt::Display for TError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TError::IO(e) =>
                write!(f, "Error in IO: {}", e),
            TError::Bincode(e) =>
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
            TError::Bincode(ref e) => Some(e),
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

impl From<bincode::Error> for TError {
    fn from(err : bincode::Error) -> TError {
        TError::Bincode(err)
    }
}

pub type Result<T> = std::result::Result<T, TError>;
