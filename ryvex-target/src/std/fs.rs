use super::{
	error::{
		Error,
		IoError,
		IoErrorKind,
	},
	path::{
		Path,
		PathScheme,
	},
	Result,
	StdError,
};
use alloc::string::String;
use core::{
	fmt,
	str,
};

pub trait FileSystem<S: PathScheme> {
	type File: File<S>;

	fn expand(&self, raw: &Path<S>) -> Result<Path<S>>;

	fn metadata(&self, path: &Path<S>) -> Result<Metadata>;

	fn create(path: &Path<S>) -> Result<Self::File>;

	fn open(&self, path: &Path<S>, opts: OpenOptions) -> Result<Self::File>;

	fn read_to_string(&self, path: &Path<S>) -> Result<String> {
		let mut file = match self.open(path, OpenOptions::read_only()) {
			Ok(f) => f,
			Err(StdError::Io(IoError(IoErrorKind::NotFound))) => {
				let _ = Self::create(path)?;
				self.open(path, OpenOptions::read_only())?
			}
			Err(e) => return Err(e),
		};

		let mut out = String::new();
		file.read_to_string(&mut out)?;

		Ok(out)
	}

	fn write_all(&self, path: &Path<S>, bytes: &[u8]) -> Result<()> {
		let mut f = self.open(path, OpenOptions::write_truncate())?;
		f.write_all(bytes)
	}
}

pub trait File<S: PathScheme> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
	fn write(&mut self, buf: &[u8]) -> Result<usize>;
	fn flush(&mut self) -> Result<()>;

	fn read_to_string(&mut self, out: &mut String) -> Result<()> {
		let mut tmp = [0u8; 4096];
		loop {
			let n = self.read(&mut tmp)?;
			if n == 0 {
				break;
			}
			let s = str::from_utf8(&tmp[..n])?;
			out.push_str(s);
		}
		Ok(())
	}

	fn write_all(&mut self, buf: &[u8]) -> Result<()> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
	Expand(String),
}

impl Error for FsError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			FsError::Expand(_) => None,
		}
	}
}

impl fmt::Display for FsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			FsError::Expand(s) => format!("could not expand path '{}'", s),
		};

		write!(f, "{}", s)
	}
}
