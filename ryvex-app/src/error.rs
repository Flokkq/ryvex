use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

use crate::editor::document::DocumentError;

#[derive(StackTraceDebug)]
pub enum RyvexError {
	StdError(ryvex_std::error::StdError),
	TermError(ryvex_term::error::TermError),
	DocumentError(DocumentError),
	LoggerError(String),
	ArgParseError(String),
}

impl Error for RyvexError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RyvexError::StdError(error) => Some(error),
			RyvexError::TermError(error) => Some(error),
			RyvexError::DocumentError(error) => Some(error),
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
			RyvexError::StdError(err) => write!(f, "ryvex-std error: {}", err),
			RyvexError::TermError(err) => {
				write!(f, "ryvex-term error: {}", err)
			}
			RyvexError::DocumentError(err) => {
				write!(f, "Document error: {}", err)
			}
			RyvexError::LoggerError(msg) => {
				write!(f, "Error while initializing logger: {}", msg)
			}
			RyvexError::ArgParseError(msg) => {
				write!(f, "Error while parsing arguments: {}", msg)
			}
		}
	}
}

impl From<ryvex_std::error::StdError> for RyvexError {
	fn from(error: ryvex_std::error::StdError) -> Self {
		RyvexError::StdError(error)
	}
}

impl From<ryvex_term::error::TermError> for RyvexError {
	fn from(error: ryvex_term::error::TermError) -> Self {
		RyvexError::TermError(error)
	}
}

impl From<DocumentError> for RyvexError {
	fn from(error: DocumentError) -> Self {
		RyvexError::DocumentError(error)
	}
}

pub type Result<T> = std::result::Result<T, RyvexError>;
