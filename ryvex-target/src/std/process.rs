use std::fmt::Display;

use super::{
	error::Error,
	Result,
	StdError,
};

pub trait Shell {
	/// Executes a command and returns its status
	fn status(&self, cmd: &str, args: &[&str]) -> Result<Exitstatus>;
}

#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exitstatus {
	Success = 0,
	Failure = 1,
}

impl Exitstatus {
	pub fn from_code(code: i32) -> Result<Self> {
		match code {
			x if x == Exitstatus::Success as i32 => Ok(Exitstatus::Success),
			x if x == Exitstatus::Failure as i32 => Ok(Exitstatus::Failure),
			_ => Err(ShellError::InvalidExitCode(code).into()),
		}
	}

	pub fn success(&self) -> bool {
		self == &Self::Success
	}

	pub fn failure(&self) -> bool {
		self == &Self::Failure
	}
}

impl TryFrom<i32> for Exitstatus {
	type Error = StdError;

	fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
		Exitstatus::from_code(value)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellError {
	ExecutionFailed(String),
	CommandNotFound(String),
	InvalidExitCode(i32),
}

impl Error for ShellError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ShellError::ExecutionFailed(_) => None,
			ShellError::CommandNotFound(_) => None,
			ShellError::InvalidExitCode(_) => None,
		}
	}
}

impl Display for ShellError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			ShellError::ExecutionFailed(s) => {
				format!("failed executing command '{}'", s)
			}
			ShellError::CommandNotFound(s) => {
				format!("command '{}' not found", s)
			}
			ShellError::InvalidExitCode(i) => {
				format!("recieved invalid exit code '{}'", i)
			}
		};

		write!(f, "{}", s)
	}
}
