use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

#[derive(StackTraceDebug)]
pub enum StdError {
	IoError(std::io::Error),
}

impl Error for StdError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			StdError::IoError(error) => Some(error),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl From<std::io::Error> for StdError {
	fn from(value: std::io::Error) -> Self {
		Self::IoError(value)
	}
}

impl Display for StdError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StdError::IoError(_) => write!(f, "IO error"),
		}
	}
}

pub type Result<T> = std::result::Result<T, StdError>;
