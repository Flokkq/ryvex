pub mod env;
pub mod fs;
pub mod process;

use std::io::ErrorKind;

use env::StdEnv;
use fs::StdFileHandle;
use fs::StdFileSystem;
use process::StdShell;

use crate::std::error::IoError;
use crate::std::error::IoErrorKind;
use crate::std::path::Path;
use crate::target::TargetPathScheme;

pub type TargetEnvironment = StdEnv<TargetPathScheme>;
pub type TargetFileHandle = StdFileHandle<TargetPathScheme>;
pub type TargetFileSystem = StdFileSystem<TargetPathScheme>;
pub type TargetShell = StdShell<TargetPathScheme>;
pub type TargetPath = Path<TargetPathScheme>;

#[derive(Debug, Clone, Default)]
pub struct TargetContext {
	pub env:   TargetEnvironment,
	pub fs:    TargetFileSystem,
	pub shell: TargetShell,
}

impl From<std::io::Error> for IoError {
	fn from(err: std::io::Error) -> IoError {
		let kind = match err.kind() {
			ErrorKind::NotFound => IoErrorKind::NotFound,
			ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
			ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
			ErrorKind::InvalidInput => IoErrorKind::InvalidInput,
			ErrorKind::UnexpectedEof => IoErrorKind::UnexpectedEof,
			ErrorKind::WouldBlock => IoErrorKind::WouldBlock,
			_ => IoErrorKind::Other,
		};

		IoError(kind)
	}
}
