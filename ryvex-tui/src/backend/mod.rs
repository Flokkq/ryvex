pub mod term;
#[cfg(test)]
pub mod test;

use std::{
	error::Error,
	fmt::Display,
};

use proc_macros::StackTraceDebug;
use ryvex_ui::graphics::{
	CursorKind,
	Rect,
};

use crate::buffer::Cell;

pub trait Backend {
	fn draw<'a, I>(&mut self, content: I) -> Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a Cell)>;
	fn hide_cursor(&mut self) -> Result<()>;
	fn show_cursor(&mut self, kind: CursorKind) -> Result<()>;
	fn get_cursor(&mut self) -> Result<(u16, u16)>;
	fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;
	fn clear(&mut self) -> Result<()>;
	fn size(&self) -> Result<Rect>;
	fn flush(&mut self) -> Result<()>;
}

#[derive(StackTraceDebug)]
pub enum BackendError {
	IOError(std::io::Error),
}

impl Error for BackendError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			BackendError::IOError(error) => Some(error),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for BackendError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BackendError::IOError(_) => write!(f, "IO error"),
		}
	}
}

impl From<std::io::Error> for BackendError {
	fn from(value: std::io::Error) -> Self {
		Self::IOError(value)
	}
}

pub type Result<T> = std::result::Result<T, BackendError>;
