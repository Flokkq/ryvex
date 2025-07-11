pub mod env;
pub mod fs;
pub mod path;
pub mod process;

#[derive(Debug)]
pub enum StdError {
	Fs(fs::FsError),
	Shell(process::ShellError),
}

pub type Result<T> = core::result::Result<T, StdError>;
