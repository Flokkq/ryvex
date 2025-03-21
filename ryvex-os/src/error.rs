use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

#[derive(StackTraceDebug)]
pub enum OsError {
	TermiosError(std::io::Error),
}

impl Error for OsError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			OsError::TermiosError(error) => Some(error),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for OsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			OsError::TermiosError(err) => write!(f, "termios error: {}", err),
		}
	}
}

impl From<std::io::Error> for OsError {
	fn from(error: std::io::Error) -> Self {
		OsError::TermiosError(error)
	}
}

pub type Result<T> = std::result::Result<T, OsError>;
