use std::{
	io::ErrorKind,
	marker::PhantomData,
	process::{
		Command,
		Stdio,
	},
};

use crate::std::{
	error::IoError,
	path::PathScheme,
	process::{
		Exitstatus,
		Shell,
		ShellError,
	},
	Result,
	StdError,
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
	fn status(&self, cmd: &str, args: &[&str]) -> Result<Exitstatus> {
		let code = Command::new(cmd)
			.args(args)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.status()
			.map_err(|err| -> StdError {
				if err.kind() == ErrorKind::NotFound {
					ShellError::CommandNotFound(cmd.to_string()).into()
				} else {
					IoError::from(err).into()
				}
			})?
			.code()
			.ok_or(ShellError::ExecutionFailed(cmd.to_string()))?;

		Exitstatus::from_code(code)
	}
}
