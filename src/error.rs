use std::borrow::ToOwned;
use std::io;

/// Represents the types of errors that may occur while using cc-rs.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Error occurred while performing I/O.
    IOError,
    /// Invalid architecture supplied.
    ArchitectureInvalid,
    /// Environment variable not found, with the var in question as extra info.
    EnvVarNotFound,
    /// Error occurred while using external tools (ie: invocation of compiler).
    ToolExecError,
    /// Error occurred due to missing external tools.
    ToolNotFound,
}

/// Represents an internal error that occurred, with an explanation.
#[derive(Clone, Debug)]
pub struct Error {
    /// Describes the kind of error that occurred.
    pub kind: ErrorKind,
    /// More explanation of error that occurred.
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind,
            message: message.to_owned(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(ErrorKind::IOError, &format!("{}", e))
    }
}

