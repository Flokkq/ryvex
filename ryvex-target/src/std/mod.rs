use core::{
	fmt::Display,
	str::Utf8Error,
};

use error::Error;

pub mod env;
pub mod error;
pub mod fs;
pub mod path;
pub mod process;
pub mod write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StdError {
	Fs(fs::FsError),
	Shell(process::ShellError),
	Io(error::IoError),
	Utf8(Utf8Error),
}

impl Error for StdError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			StdError::Fs(e) => Some(e),
			StdError::Shell(e) => Some(e),
			StdError::Io(e) => Some(e),
			StdError::Utf8(e) => Some(e),
		}
	}
}

impl Display for StdError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let s = match self {
			StdError::Fs(_) => "fs-error",
			StdError::Shell(_) => "shell error",
			StdError::Io(_) => "io error",
			StdError::Utf8(_) => "utf8 error",
		};

		write!(f, "{}", s)
	}
}

impl From<Utf8Error> for StdError {
	fn from(value: Utf8Error) -> Self {
		Self::Utf8(value)
	}
}

impl From<fs::FsError> for StdError {
	fn from(value: fs::FsError) -> Self {
		Self::Fs(value)
	}
}

impl From<process::ShellError> for StdError {
	fn from(value: process::ShellError) -> Self {
		Self::Shell(value)
	}
}

impl From<error::IoError> for StdError {
	fn from(value: error::IoError) -> Self {
		Self::Io(value)
	}
}

pub type Result<T> = core::result::Result<T, StdError>;
