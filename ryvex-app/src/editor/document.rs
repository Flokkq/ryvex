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
use ryvex_core::{
	piece_table::{
		PieceTable,
		RowCol,
	},
	MarkTable,
	TextBuffer,
};
use ryvex_target::{
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
	path:   Option<TargetPath>,

	buffer: PieceTable,
	cursor: RowCol,
	mark:   Option<RowCol>,
	marks:  MarkTable,
}

impl Default for Document {
	fn default() -> Self {
		Self::scratch()
	}
}

impl Document {
	pub fn scratch() -> Self {
		Self {
			id:     DocumentId::default(),
			path:   None,
			buffer: PieceTable::new(String::new()),
			cursor: RowCol { row: 0, col: 0 },
			mark:   None,
			marks:  MarkTable::default(),
		}
	}

	pub fn open(path: TargetPath, fs: &TargetFileSystem) -> Result<Self> {
		let content = fs.read_to_string(&path)?;

		Ok(Self {
			id:     DocumentId::default(),
			path:   Some(path),
			buffer: PieceTable::new(content),
			cursor: RowCol { row: 0, col: 0 },
			mark:   None,
			marks:  MarkTable::default(),
		})
	}

	pub fn scratch_from_string(text: String) -> Self {
		Self {
			id:     DocumentId::default(),
			path:   None,
			buffer: PieceTable::new(text),
			cursor: RowCol { row: 0, col: 0 },
			mark:   None,
			marks:  MarkTable::default(),
		}
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

	pub fn content(&self) -> String {
		self.buffer.slice(0, self.buffer.len())
	}

	pub fn save(&self, fs: &TargetFileSystem) -> Result<()> {
		match &self.path {
			Some(path) => {
				fs.write_all(path, self.content().as_bytes())?;
				Ok(())
			}
			// TODO: error because file doesnt exist
			None => Ok(()),
		}
	}

	pub fn current_row(&self) -> usize {
		self.cursor.row
	}
	pub fn current_col(&self) -> usize {
		self.cursor.col
	}
	pub fn total_lines(&self) -> usize {
		self.buffer.lines()
	}

	pub fn rows(&self) -> usize {
		self.content().lines().count()
	}

	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	pub fn is_empty(&self) -> bool {
		self.buffer.is_empty()
	}

	pub fn insert_character(&mut self, ch: char) {
		// 1) map (row,col) → byte offset
		let pos = self.buffer.pos_from(self.cursor);
		// 2) insert into piece-table
		self.buffer.insert(pos, &ch.to_string());
		// 3) advance cursor
		if ch == '\n' {
			self.cursor.row += 1;
			self.cursor.col = 0;
		} else {
			self.cursor.col += 1;
		}
	}

	pub fn path(&self) -> Option<&TargetPath> {
		self.path.as_ref()
	}

	pub fn diplay_path(&self, fs: &TargetFileSystem) -> Option<String> {
		self.path
			.clone()
			.map(|p| fs.expand(&p).unwrap_or(p).to_string())
	}

	pub fn buffer(&self) -> &PieceTable {
		&self.buffer
	}

	pub fn cursor(&self) -> RowCol {
		self.cursor
	}

	pub fn delete_at_cursor(&mut self) {
		let mut pos = self.buffer.pos_from(self.cursor);

		if pos == self.buffer.len() && pos > 0 {
			let mut start = pos - 1;
			while start > 0 && self.buffer.char_at(start).is_none() {
				start -= 1;
			}
			pos = start;
		}

		if let Some(ch) = self.buffer.char_at(pos) {
			let end = pos + ch.len_utf8();
			self.buffer.delete(pos, end);
			self.cursor = self.buffer.rowcol_at(pos);
		}
	}

	pub fn buffer_mut(&mut self) -> &mut PieceTable {
		&mut self.buffer
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
