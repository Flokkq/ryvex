use std::{
	env,
	fs::{
		self,
	},
	io::{
		Read,
		Write,
	},
	marker::PhantomData,
	path::Path as StdPath,
	str::FromStr,
};

use crate::std::{
	error::IoError,
	fs::{
		File,
		FileSystem,
		Metadata,
		OpenOptions,
	},
	path::{
		Path,
		PathScheme,
	},
	Result,
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

	fn expand(&self, raw: &Path<S>) -> Result<Path<S>> {
		let p = StdPath::new(raw.as_str());
		let abs = if p.is_absolute() {
			p.to_path_buf()
		} else {
			let mut cwd = env::current_dir().map_err(IoError::from)?;
			cwd.push(p);
			cwd
		};
		let s = abs.to_string_lossy().into_owned();
		Ok(Path::<S>::from_str(&s).unwrap())
	}

	fn metadata(&self, path: &Path<S>) -> Result<Metadata> {
		let m =
			fs::metadata(StdPath::new(path.as_str())).map_err(IoError::from)?;

		Ok(Metadata {
			is_dir: m.is_dir(),
			len:    m.len(),
		})
	}

	fn open(&self, path: &Path<S>, opts: OpenOptions) -> Result<Self::File> {
		let f = fs::OpenOptions::new()
			.read(opts.read)
			.write(opts.write)
			.create(opts.create)
			.truncate(opts.truncate)
			.open(StdPath::new(path.as_str()))
			.map_err(IoError::from)?;

		Ok(StdFileHandle {
			inner:   f,
			_scheme: PhantomData,
		})
	}

	fn create(path: &Path<S>) -> Result<Self::File> {
		let f = fs::File::create(StdPath::new(path.as_str()))
			.map_err(IoError::from)?;

		Ok(StdFileHandle {
			inner:   f,
			_scheme: PhantomData,
		})
	}
}

impl<S: PathScheme> File<S> for StdFileHandle<S> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		Ok(self.inner.read(buf).map_err(IoError::from)?)
	}
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		Ok(self.inner.write(buf).map_err(IoError::from)?)
	}
	fn flush(&mut self) -> Result<()> {
		Ok(self.inner.flush().map_err(IoError::from)?)
	}
}
