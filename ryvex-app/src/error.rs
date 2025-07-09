use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

use crate::editor::error::{
	CommandError,
	DocumentError,
};

#[derive(StackTraceDebug)]
pub enum RyvexError {
	IoError(std::io::Error),
	StdError(ryvex_std::error::StdError),
	TuiError(ryvex_tui::error::TuiError),
	DocumentError(DocumentError),
	CommandError(CommandError),
	LoggerError(String),
	ArgParseError(String),
}

impl Error for RyvexError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RyvexError::IoError(error) => Some(error),
			RyvexError::StdError(error) => Some(error),
			RyvexError::TuiError(error) => Some(error),
			RyvexError::DocumentError(error) => Some(error),
			RyvexError::CommandError(error) => Some(error),
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
			RyvexError::IoError(err) => write!(f, "io-error: {}", err),
			RyvexError::StdError(err) => write!(f, "ryvex-std error: {}", err),
			RyvexError::TuiError(err) => {
				write!(f, "ryvex-tui error: {}", err)
			}
			RyvexError::DocumentError(err) => {
				write!(f, "Document error: {}", err)
			}
			RyvexError::CommandError(err) => {
				write!(f, "Command error: {}", err)
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

impl From<std::io::Error> for RyvexError {
	fn from(value: std::io::Error) -> Self {
		RyvexError::IoError(value)
	}
}

impl From<ryvex_tui::error::TuiError> for RyvexError {
	fn from(error: ryvex_tui::error::TuiError) -> Self {
		RyvexError::TuiError(error)
	}
}

impl From<DocumentError> for RyvexError {
	fn from(error: DocumentError) -> Self {
		RyvexError::DocumentError(error)
	}
}

impl From<CommandError> for RyvexError {
	fn from(error: CommandError) -> Self {
		RyvexError::CommandError(error)
	}
}

pub type Result<T> = std::result::Result<T, RyvexError>;
