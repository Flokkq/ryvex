use proc_macros::StackTraceDebug;
use std::{
	error::Error,
	fmt::Display,
};

#[derive(StackTraceDebug)]
pub enum DocumentError {
	SaveError(std::io::Error),
	OpenError(std::io::Error),
}

impl Error for DocumentError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			DocumentError::SaveError(err) => Some(err),
			DocumentError::OpenError(err) => Some(err),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for DocumentError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DocumentError::SaveError(err) => {
				write!(f, "Failed to save document: {}", err)
			}
			DocumentError::OpenError(err) => {
				write!(f, "Failed to open document: {}", err)
			}
		}
	}
}

#[derive(StackTraceDebug)]
pub enum CommandError {
	InvalidCommand,
	ExecutionFailed,
	Unexpected,
}

impl Display for CommandError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CommandError::InvalidCommand => {
				write!(f, "Invalid command provided")
			}
			CommandError::ExecutionFailed => {
				write!(f, "Failed executing command")
			}
			CommandError::Unexpected => {
				write!(f, "Encoutered Unexpected error executing command")
			}
		}
	}
}

impl std::error::Error for CommandError {}
