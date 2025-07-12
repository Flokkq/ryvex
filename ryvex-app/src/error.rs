use alloc::{
	format,
	string::String,
};
use core::fmt::{
	self,
	Display,
};

use ryvex_target::std::{
	error::Error,
	StdError,
};

#[derive(Clone, PartialEq)]
pub enum RyvexError {
	StdError(StdError),
	LoggerError(String),
	ArgParseError(String),
}

impl Error for RyvexError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RyvexError::StdError(err) => Some(err),
			RyvexError::LoggerError(_) => None,
			RyvexError::ArgParseError(_) => None,
		}
	}
}

impl Display for RyvexError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			RyvexError::StdError(_) => "std error",
			RyvexError::LoggerError(msg) => {
				&format!("failed initializing logger '{}'", msg)
			}
			RyvexError::ArgParseError(msg) => {
				&format!("failed parsing arguments '{}'", msg)
			}
		};

		write!(f, "{}", s)
	}
}

impl From<StdError> for RyvexError {
	fn from(value: StdError) -> Self {
		Self::StdError(value)
	}
}

pub type Result<T> = core::result::Result<T, RyvexError>;
