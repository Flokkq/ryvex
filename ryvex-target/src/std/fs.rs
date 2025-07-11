use alloc::string::String;
use core::{
	fmt,
	result::Result,
};

use super::path::{
	Path,
	PathScheme,
};

pub trait FileSystem<S: PathScheme> {
	type File: File<S>;

	fn expand(&self, raw: &Path<S>) -> Result<Path<S>, FsError>;

	fn metadata(&self, path: &Path<S>) -> Result<Metadata, FsError>;

	fn create(path: &Path<S>) -> Result<Self::File, FsError>;

	fn open(
		&self,
		path: &Path<S>,
		opts: OpenOptions,
	) -> Result<Self::File, FsError>;

	fn read_to_string(&self, path: &Path<S>) -> Result<String, FsError> {
		let mut file = match self.open(path, OpenOptions::read_only()) {
			Ok(f) => f,
			Err(FsError::Io(IoErrorKind::NotFound)) => {
				let _ = Self::create(path)?;
				self.open(path, OpenOptions::read_only())?
			}
			Err(e) => return Err(e),
		};

		let mut out = String::new();
		file.read_to_string(&mut out)?;

		Ok(out)
	}

	fn write_all(&self, path: &Path<S>, bytes: &[u8]) -> Result<(), FsError> {
		let mut f = self.open(path, OpenOptions::write_truncate())?;
		f.write_all(bytes)
	}
}

pub trait File<S: PathScheme> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, FsError>;
	fn write(&mut self, buf: &[u8]) -> Result<usize, FsError>;
	fn flush(&mut self) -> Result<(), FsError>;

	fn read_to_string(&mut self, out: &mut String) -> Result<(), FsError> {
		let mut tmp = [0u8; 4096];
		loop {
			let n = self.read(&mut tmp)?;
			if n == 0 {
				break;
			}
			let s = core::str::from_utf8(&tmp[..n])?;
			out.push_str(s);
		}
		Ok(())
	}

	fn write_all(&mut self, buf: &[u8]) -> Result<(), FsError> {
		let mut off = 0;
		while off < buf.len() {
			off += self.write(&buf[off..])?;
		}
		self.flush()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Metadata {
	pub is_dir: bool,
	pub len:    u64,
}

#[derive(Clone, Copy, Debug)]
pub struct OpenOptions {
	pub read:     bool,
	pub write:    bool,
	pub create:   bool,
	pub truncate: bool,
}

impl OpenOptions {
	pub fn read_only() -> Self {
		Self {
			read:     true,
			write:    false,
			create:   false,
			truncate: false,
		}
	}
	pub fn write_truncate() -> Self {
		Self {
			read:     false,
			write:    true,
			create:   false,
			truncate: true,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
	Io(IoErrorKind),
	Utf8,
	Expand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoErrorKind {
	NotFound,
	PermissionDenied,
	AlreadyExists,
	InvalidInput,
	UnexpectedEof,
	WouldBlock,
	Other,
}

impl fmt::Display for FsError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			FsError::Io(kind) => write!(f, "I/O error: {}", kind),
			FsError::Utf8 => write!(f, "invalid UTF-8 sequence"),
			FsError::Expand => write!(f, "path expansion error"),
		}
	}
}

impl fmt::Display for IoErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match *self {
			IoErrorKind::NotFound => "not found",
			IoErrorKind::PermissionDenied => "permission denied",
			IoErrorKind::AlreadyExists => "already exists",
			IoErrorKind::InvalidInput => "invalid input",
			IoErrorKind::UnexpectedEof => "unexpected EOF",
			IoErrorKind::WouldBlock => "would block",
			IoErrorKind::Other => "other I/O error",
		};
		write!(f, "{}", s)
	}
}

impl From<core::str::Utf8Error> for FsError {
	fn from(_: core::str::Utf8Error) -> FsError {
		FsError::Utf8
	}
}
