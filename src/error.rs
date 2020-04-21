use std::{
    error::Error,
    fmt::{self, Display},
    io,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum NbsError {
    /// This error occures when the format does not contain the expected data
    InvalidFormat,
    /// This error occurs when decoding a string thats not utf-8
    InvalidString(FromUtf8Error),
    /// This error occures when an io operation fails
    IoError(io::Error),
}

impl From<io::Error> for NbsError {
    fn from(e: io::Error) -> Self {
        NbsError::IoError(e)
    }
}

impl From<FromUtf8Error> for NbsError {
    fn from(e: FromUtf8Error) -> Self {
        NbsError::InvalidString(e)
    }
}

impl Display for NbsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NbsError::InvalidFormat => {
                write!(f, "The target format is not supported by the given data.")
            }
            NbsError::InvalidString(e) => write!(f, "Failed to decode string; {}", e),
            NbsError::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for NbsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NbsError::InvalidFormat => None,
            NbsError::InvalidString(e) => Some(e),
            NbsError::IoError(e) => Some(e),
        }
    }
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
