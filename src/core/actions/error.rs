use std::fmt;

#[derive(Debug)]
pub enum ActionError {
    InvalidCommand,
    ExecutionFailed,
    Unexpected,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ActionError::InvalidCommand => write!(f, "Invalid command"),
            ActionError::ExecutionFailed => {
                write!(f, "Failed to execute command")
            }
            ActionError::Unexpected => {
                write!(f, "Unexpected Error")
            }
        }
    }
}

impl std::error::Error for ActionError {}
