use std::fmt;

use crate::core::actions::error::ActionError;

use super::ui::error::OverlayError;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Action(ActionError),
    Overlay(OverlayError),
    Unexpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Action(ref err) => write!(f, "Action error: {}", err),
            Error::Overlay(ref err) => write!(f, "Overlay error: {}", err),
            Error::Unexpected => write!(f, "Unexpected error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Action(ref err) => Some(err),
            Error::Overlay(ref err) => Some(err),
            Error::Unexpected => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ActionError> for Error {
    fn from(err: ActionError) -> Self {
        Error::Action(err)
    }
}

impl From<OverlayError> for Error {
    fn from(err: OverlayError) -> Self {
        Error::Overlay(err)
    }
}
