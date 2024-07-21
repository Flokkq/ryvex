use std::fmt;

use crate::core::actions::error::ActionError;

#[derive(Debug)]
pub enum OverlayError {
    Io(std::io::Error),
    CommandError(ActionError),
    Unexpected,
}

impl fmt::Display for OverlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OverlayError::Io(ref err) => write!(f, "IO Error: {}", err),
            OverlayError::CommandError(ref err) => {
                write!(f, "Command Execution Error: {}", err)
            }
            OverlayError::Unexpected => write!(f, "Unexpected error"),
        }
    }
}

impl std::error::Error for OverlayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            OverlayError::Io(ref err) => Some(err),
            OverlayError::CommandError(ref err) => Some(err),
            OverlayError::Unexpected => None,
        }
    }
}

impl From<std::io::Error> for OverlayError {
    fn from(err: std::io::Error) -> Self {
        OverlayError::Io(err)
    }
}
