#![cfg_attr(not(feature = "std"), no_std)]

use piece_table::RowCol;
pub extern crate alloc;

pub mod logging;
pub mod motion;
pub mod piece_table;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
	pub pos: usize,
}

impl Cursor {
	pub fn clamp(&mut self, buf: &impl TextBuffer) {
		self.pos = self.pos.min(buf.len());
	}
}

pub trait TextBuffer {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn char_at(&self, idx: usize) -> Option<char>;
	fn slice(&self, start: usize, end: usize) -> String;

	fn insert(&mut self, idx: usize, text: &str);
	fn delete(&mut self, start: usize, end: usize);
	fn find(&self, pattern: &str, from: usize) -> Option<usize>;

	fn rowcol_at(&self, idx: usize) -> RowCol;
	fn pos_from(&self, rc: RowCol) -> usize;
	fn lines(&self) -> usize;
	fn line_len(&self, row: usize) -> usize;
}

#[derive(Default, Clone, Debug)]
pub struct MarkTable {
	slots: [Option<RowCol>; 26],
}

impl MarkTable {
	pub fn set(&mut self, ch: char, pos: RowCol) {
		if ch.is_ascii_lowercase() {
			self.slots[(ch as u8 - b'a') as usize] = Some(pos);
		}
	}
	pub fn get(&self, ch: char) -> Option<RowCol> {
		if ch.is_ascii_lowercase() {
			self.slots[(ch as u8 - b'a') as usize]
		} else {
			None
		}
	}
}
