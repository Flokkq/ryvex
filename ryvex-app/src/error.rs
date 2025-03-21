use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

#[derive(StackTraceDebug)]
pub enum RyvexError {
	OsError(ryvex_os::error::OsError),
	LoggerError(String),
	ArgParseError(String),
}

impl Error for RyvexError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RyvexError::OsError(error) => Some(error),
			RyvexError::LoggerError(_) => None,
			RyvexError::ArgParseError(_) => None,
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for RyvexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RyvexError::OsError(err) => write!(f, "ryvex-os error: {}", err),
			RyvexError::LoggerError(msg) => {
				write!(f, "Error while initializing logger: {}", msg)
			}
			RyvexError::ArgParseError(msg) => {
				write!(f, "Error while parsing arguments: {}", msg)
			}
		}
	}
}

impl From<ryvex_os::error::OsError> for RyvexError {
	fn from(error: ryvex_os::error::OsError) -> Self {
		RyvexError::OsError(error)
	}
}

pub type Result<T> = std::result::Result<T, RyvexError>;
