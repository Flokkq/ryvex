use super::fs::IoErrorKind;

pub trait Shell {
	fn status(&self, cmd: &str, args: &[&str]) -> Result<i32, ShellError>;
}

#[derive(Debug)]
pub enum ShellError {
	ExecutionFailed,
	Io(IoErrorKind),
}
