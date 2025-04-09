use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;

use crate::backend::BackendError;

#[derive(StackTraceDebug)]
pub enum TuiError {
	BackendError(BackendError),
}

impl Error for TuiError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TuiError::BackendError(error) => Some(error),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for TuiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TuiError::BackendError(err) => write!(f, "Backend Error: {}", err),
		}
	}
}

impl From<BackendError> for TuiError {
	fn from(value: BackendError) -> Self {
		Self::BackendError(value)
	}
}

pub type Result<T> = std::result::Result<T, TuiError>;
