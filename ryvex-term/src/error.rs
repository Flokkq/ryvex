use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

#[derive(StackTraceDebug)]
pub enum TermError {
	IoError(std::io::Error),
	TermiosError(std::io::Error),
}

impl Error for TermError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TermError::IoError(error) => Some(error),
			TermError::TermiosError(error) => Some(error),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl From<std::io::Error> for TermError {
	fn from(value: std::io::Error) -> Self {
		Self::IoError(value)
	}
}

impl Display for TermError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TermError::IoError(_) => write!(f, "IO error"),
			TermError::TermiosError(_) => write!(f, "Termios error"),
		}
	}
}

pub type Result<T> = std::result::Result<T, TermError>;
