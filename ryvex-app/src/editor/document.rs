use core::{
	fmt::{
		self,
		Display,
	},
	num::NonZeroUsize,
};

use alloc::string::{
	String,
	ToString,
};
use ryvex_target::{
	key,
	r#impl::{
		TargetFileSystem,
		TargetPath,
	},
	std::fs::FileSystem,
};

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

impl core::fmt::Display for DocumentId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("{}", self.0))
	}
}

#[derive(Debug)]
pub struct Document {
	pub id: DocumentId,
	text:   String,
	path:   Option<TargetPath>,
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

	pub fn open(path: TargetPath, fs: &TargetFileSystem) -> Result<Self> {
		let content = fs.read_to_string(&path).unwrap();

		Ok(Self {
			id:   DocumentId::default(),
			text: content,
			path: Some(path),
		})
	}

	pub fn new(
		path: Option<TargetPath>,
		fs: &TargetFileSystem,
	) -> Result<Self> {
		match path {
			Some(path) => Self::open(path, fs),
			None => Ok(Self::scratch()),
		}
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	pub fn save(&self, fs: &TargetFileSystem) -> Result<()> {
		match &self.path {
			Some(path) => {
				fs.write_all(path, self.text.as_bytes()).unwrap();
				Ok(())
			}
			// TODO: error because file doesnt exist
			None => Ok(()),
		}
	}

	pub fn insert_character(&mut self, key: key::AsciiKeyCode) {
		self.text.push(key.to_char());
	}

	pub fn path(&self) -> Option<&TargetPath> {
		self.path.as_ref()
	}

	pub fn diplay_path(&self, fs: &TargetFileSystem) -> Option<String> {
		self.path
			.clone()
			.map(|p| fs.expand(&p).unwrap_or(p).to_string())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
	Normal = 0,
	Visual = 1,
	Insert = 2,
	Command = 3,
}

impl Display for Mode {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Mode::Normal => f.write_str("NORMAL"),
			Mode::Visual => f.write_str("VISUAL"),
			Mode::Insert => f.write_str("INSERT"),
			Mode::Command => f.write_str("COMMAND"),
		}
	}
}
