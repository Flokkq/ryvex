pub mod env;
pub mod fs;
pub mod process;

use core::str::FromStr;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

use env::StdEnv;
use fs::StdFileHandle;
use fs::StdFileSystem;
use process::StdShell;

use crate::std::error::IoError;
use crate::std::error::IoErrorKind;
use crate::std::fs::FileSystem;
use crate::std::path::Path;
use crate::std::path::PathScheme;
use crate::std::write::Write;
use crate::std::StdError;
use crate::target::TargetPathScheme;

pub type TargetEnvironment = StdEnv<TargetPathScheme>;
pub type TargetFileHandle = StdFileHandle<TargetPathScheme>;
pub type TargetFileSystem = StdFileSystem<TargetPathScheme>;
pub type TargetShell = StdShell<TargetPathScheme>;
pub type TargetPath = Path<TargetPathScheme>;
pub type TargetOutWriter = StdOutWriter;
pub type TargetLoggingWriter = LoggingWriter;

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
			ErrorKind::Interrupted => IoErrorKind::Interrupted,
			_ => IoErrorKind::Other,
		};

		IoError(kind)
	}
}

#[derive(Debug)]
pub struct StdOutWriter(std::io::Stdout);

impl Default for StdOutWriter {
	fn default() -> Self {
		Self(std::io::stdout())
	}
}

impl Write for StdOutWriter {
	fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
		use std::io::Write as _;

		self.0.write(buf).map_err(IoError::from)
	}

	fn flush(&mut self) -> Result<(), IoError> {
		use std::io::Write as _;

		self.0.flush().map_err(IoError::from)
	}
}

pub fn exit(code: i32) -> ! {
	std::process::exit(code)
}

#[derive(Debug)]
pub struct LoggingWriter(File);

impl LoggingWriter {
	pub fn try_init(fs: &TargetFileSystem) -> crate::std::Result<Self> {
		let log_dir = PathBuf::from(TargetPathScheme::LOG_DIR_BASE);

		let path = Path::from_str(&log_dir.to_string_lossy())
			.map_err(|_| StdError::Io(IoError(IoErrorKind::InvalidInput)))?;
		let mut path = fs.canonicalize(&path)?;
		path.push(TargetPathScheme::LOG_DIR);

		match fs.metadata(&path) {
			Ok(_) => {}
			Err(StdError::Io(IoError(IoErrorKind::NotFound))) => {
				fs.create_dir_all(&path)?;
			}
			Err(e) => return Err(e),
		}
		path.push("ryvex.log");

		let file = fs.create(&path)?;

		Ok(Self(file.into_inner()))
	}
}

impl Write for LoggingWriter {
	fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
		use std::io::Write as _;
		self.0.write(buf).map_err(IoError::from)
	}

	fn flush(&mut self) -> Result<(), IoError> {
		use std::io::Write as _;
		self.0.flush().map_err(IoError::from)
	}
}
