use std::{
	env,
	fs::{
		self,
	},
	io::{
		self,
		Read,
		Write,
	},
	marker::PhantomData,
	path::Path as StdPath,
	str::FromStr,
};

use crate::std::{
	fs::{
		File,
		FileSystem,
		FsError,
		IoErrorKind,
		Metadata,
		OpenOptions,
	},
	path::{
		Path,
		PathScheme,
	},
};

#[derive(Debug, Clone, Default)]
pub struct StdFileSystem<S: PathScheme> {
	_scheme: PhantomData<S>,
}

impl<S: PathScheme> StdFileSystem<S> {
	pub fn new() -> Self {
		StdFileSystem {
			_scheme: PhantomData,
		}
	}
}

pub struct StdFileHandle<S: PathScheme> {
	inner:   fs::File,
	_scheme: PhantomData<S>,
}

impl<S: PathScheme> FileSystem<S> for StdFileSystem<S> {
	type File = StdFileHandle<S>;

	fn expand(&self, raw: &Path<S>) -> Result<Path<S>, FsError> {
		let p = StdPath::new(raw.as_str());
		let abs = if p.is_absolute() {
			p.to_path_buf()
		} else {
			let mut cwd = env::current_dir()?;
			cwd.push(p);
			cwd
		};
		let s = abs.to_string_lossy().into_owned();
		Ok(Path::<S>::from_str(&s).unwrap())
	}

	fn metadata(&self, path: &Path<S>) -> Result<Metadata, FsError> {
		let m = fs::metadata(StdPath::new(path.as_str()))?;

		Ok(Metadata {
			is_dir: m.is_dir(),
			len:    m.len(),
		})
	}

	fn open(
		&self,
		path: &Path<S>,
		opts: OpenOptions,
	) -> Result<Self::File, FsError> {
		let f = fs::OpenOptions::new()
			.read(opts.read)
			.write(opts.write)
			.create(opts.create)
			.truncate(opts.truncate)
			.open(StdPath::new(path.as_str()))?;

		Ok(StdFileHandle {
			inner:   f,
			_scheme: PhantomData,
		})
	}

	fn create(path: &Path<S>) -> Result<Self::File, FsError> {
		let f = fs::File::create(StdPath::new(path.as_str()))?;

		Ok(StdFileHandle {
			inner:   f,
			_scheme: PhantomData,
		})
	}
}

impl<S: PathScheme> File<S> for StdFileHandle<S> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, FsError> {
		self.inner.read(buf).map_err(FsError::from)
	}
	fn write(&mut self, buf: &[u8]) -> Result<usize, FsError> {
		self.inner.write(buf).map_err(FsError::from)
	}
	fn flush(&mut self) -> Result<(), FsError> {
		self.inner.flush().map_err(FsError::from)
	}
}

impl From<io::Error> for FsError {
	fn from(err: io::Error) -> FsError {
		let kind = match err.kind() {
			io::ErrorKind::NotFound => IoErrorKind::NotFound,
			io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
			io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
			io::ErrorKind::InvalidInput => IoErrorKind::InvalidInput,
			io::ErrorKind::UnexpectedEof => IoErrorKind::UnexpectedEof,
			io::ErrorKind::WouldBlock => IoErrorKind::WouldBlock,
			_ => IoErrorKind::Other,
		};

		FsError::Io(kind)
	}
}
