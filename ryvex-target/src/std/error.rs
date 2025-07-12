use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

/// Simple trait for chaining errors
pub trait Error: Display {
	fn source(&self) -> Option<&(dyn Error + 'static)>;

	fn root(&self) -> Option<&(dyn Error + 'static)>
	where
		Self: 'static + Sized,
	{
		let mut current: &(dyn Error + 'static) = self;
		let mut last: Option<&(dyn Error + 'static)> = None;

		while let Some(src) = current.source() {
			last = Some(src);
			current = src;
		}

		last
	}
}

impl fmt::Debug for dyn Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "{}", self)?;

		let mut current = self.source();
		while let Some(cause) = current {
			writeln!(f, "Caused by:\n\t{}", cause)?;
			current = cause.source();
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoError(pub IoErrorKind);

impl Error for IoError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

impl Display for IoError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "io error: {}", self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoErrorKind {
	NotFound,
	PermissionDenied,
	AlreadyExists,
	InvalidInput,
	UnexpectedEof,
	Interrupted,
	WouldBlock,
	Other,
}

impl fmt::Display for IoErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match *self {
			IoErrorKind::NotFound => "not found",
			IoErrorKind::PermissionDenied => "permission denied",
			IoErrorKind::AlreadyExists => "already exists",
			IoErrorKind::InvalidInput => "invalid input",
			IoErrorKind::UnexpectedEof => "unexpected EOF",
			IoErrorKind::Interrupted => "interrupted",
			IoErrorKind::WouldBlock => "would block",
			IoErrorKind::Other => "other error",
		};
		write!(f, "{}", s)
	}
}

impl Error for core::str::Utf8Error {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}
