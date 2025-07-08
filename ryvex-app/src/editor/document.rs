use std::{
	error::Error,
	fmt::Display,
	num::NonZeroUsize,
	path::{
		Path,
		PathBuf,
	},
};

use log::warn;
use proc_macros::StackTraceDebug;
use ryvex_std::error::StdError;

use crate::error::Result;

// uses NonZeroUsize so Option<DocumentId> use a byte rather than two
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentId(pub NonZeroUsize);

impl Default for DocumentId {
	fn default() -> Self {
		// Safety: 1 is non-zero
		DocumentId(unsafe { NonZeroUsize::new_unchecked(1) })
	}
}

impl std::fmt::Display for DocumentId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.0))
	}
}

pub struct Document {
	pub id: DocumentId,
	text:   String,
	path:   Option<PathBuf>,
}

impl Default for Document {
	fn default() -> Self {
		Self::scratch()
	}
}

impl Document {
	pub fn scratch() -> Self {
		Self {
			id:   DocumentId::default(),
			text: String::new(),
			path: None,
		}
	}

	pub fn open(path: PathBuf) -> Result<Self> {
		let mut content = String::new();
		ryvex_std::fs::read_from_file_if_exists(&path, &mut content)
			.map_err(DocumentError::OpenError)?;

		// TODO: does this break on windows?
		content = content.replace("\n", "\r\n");

		Ok(Self {
			id:   DocumentId::default(),
			text: content,
			path: Some(path),
		})
	}

	pub fn new(path: Option<PathBuf>) -> Result<Self> {
		match path {
			Some(path) => Self::open(path),
			None => Ok(Self::scratch()),
		}
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	pub fn save(&self) -> Result<()> {
		match &self.path {
			Some(path) => ryvex_std::fs::write(&self.text, path)
				.map_err(|err| DocumentError::SaveError(err).into()),
			None => {
				warn!("Attempted to save document with no path");
				Ok(())
			}
		}
	}

	pub fn insert_character(&mut self, key: ryvex_term::key::AsciiKeyCode) {
		self.text.push(key.to_char());
	}

	pub fn path(&self) -> Option<&PathBuf> {
		self.path.as_ref().map(|p| p)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
	Normal = 0,
	Visual = 1,
	Insert = 2,
}

impl Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Mode::Normal => f.write_str("NORMAL"),
			Mode::Visual => f.write_str("VISUAL"),
			Mode::Insert => f.write_str("INSERT"),
		}
	}
}

#[derive(StackTraceDebug)]
pub enum DocumentError {
	SaveError(StdError),
	OpenError(StdError),
}

impl Error for DocumentError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			DocumentError::SaveError(err) => Some(err),
			DocumentError::OpenError(err) => Some(err),
		}
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}

impl Display for DocumentError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DocumentError::SaveError(err) => {
				write!(f, "Failed to save document: {}", err)
			}
			DocumentError::OpenError(err) => {
				write!(f, "Failed to open document: {}", err)
			}
		}
	}
}
