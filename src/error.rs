use std::{fmt, io};

pub enum RLiteError {
    Io(io::Error),
    MetaCommmandError(String),
    StatementError(String),
}

impl fmt::Display for RLiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RLiteError::Io(ref err) => write!(f, "IO Error: {}", err),
            RLiteError::MetaCommmandError(ref s) => write!(f, "[Meta Command Error]: err: {s}"),
            RLiteError::StatementError(ref s) => write!(f, "[Statement Error]: err: {s}"),
        }
    }
}

impl From<io::Error> for RLiteError {
    fn from(err: io::Error) -> RLiteError {
        RLiteError::Io(err)
    }
}
