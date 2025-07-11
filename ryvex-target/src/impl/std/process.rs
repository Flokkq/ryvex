use std::{
	io,
	marker::PhantomData,
	process::{
		Command,
		Stdio,
	},
};

use crate::std::{
	fs::IoErrorKind,
	path::PathScheme,
	process::{
		Shell,
		ShellError,
	},
};

#[derive(Debug, Clone, Default)]
pub struct StdShell<S: PathScheme> {
	_scheme: PhantomData<S>,
}

impl<S: PathScheme> StdShell<S> {
	pub fn new() -> Self {
		StdShell {
			_scheme: PhantomData,
		}
	}
}

impl<S: PathScheme> Shell for StdShell<S> {
	fn status(&self, cmd: &str, args: &[&str]) -> Result<i32, ShellError> {
		let status = Command::new(cmd)
			.args(args)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.status()?;

		status.code().ok_or(ShellError::ExecutionFailed)
	}
}

impl From<io::Error> for ShellError {
	fn from(e: io::Error) -> Self {
		ShellError::Io(match e.kind() {
			io::ErrorKind::NotFound => IoErrorKind::NotFound,
			io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
			io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
			io::ErrorKind::InvalidInput => IoErrorKind::InvalidInput,
			io::ErrorKind::UnexpectedEof => IoErrorKind::UnexpectedEof,
			io::ErrorKind::WouldBlock => IoErrorKind::WouldBlock,
			_ => IoErrorKind::Other,
		})
	}
}
