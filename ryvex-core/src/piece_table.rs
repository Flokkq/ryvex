use alloc::{
	string::String,
	sync::Arc,
	vec::Vec,
};

use crate::TextBuffer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowCol {
	pub row: usize,
	pub col: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BufferKind {
	Original,
	Add,
}

#[derive(Clone, Debug)]
struct Piece {
	source: BufferKind,
	start:  usize,
	len:    usize,
}

#[derive(Debug)]
pub struct PieceTable {
	original:    Arc<String>,
	add:         String,
	pieces:      Vec<Piece>,
	line_starts: Vec<usize>,
}

impl PieceTable {
	pub fn new(initial: String) -> Self {
		let mut pt = PieceTable {
			original:    Arc::new(initial.clone()),
			add:         String::new(),
			pieces:      vec![Piece {
				source: BufferKind::Original,
				start:  0,
				len:    initial.len(),
			}],
			line_starts: Vec::new(),
		};
		pt.rebuild_line_index();
		pt
	}

	fn buffer_slice(&self, piece: &Piece) -> &str {
		match piece.source {
			BufferKind::Original => {
				&self.original[piece.start..piece.start + piece.len]
			}
			BufferKind::Add => &self.add[piece.start..piece.start + piece.len],
		}
	}

	fn push_bytes(
		&self,
		piece: &Piece,
		rel_start: usize,
		rel_end: usize,
		out: &mut String,
	) {
		let slice = &self.buffer_slice(piece)[rel_start..rel_end];
		out.push_str(slice);
	}

	fn split_piece(&mut self, idx: usize, offset: usize) {
		let p = &self.pieces[idx];
		if offset == 0 || offset >= p.len {
			return;
		}
		let right = Piece {
			source: p.source,
			start:  p.start + offset,
			len:    p.len - offset,
		};
		self.pieces[idx].len = offset;
		self.pieces.insert(idx + 1, right);
	}

	fn locate(&self, idx: usize) -> (usize, usize) {
		let mut cur = 0;
		for (i, p) in self.pieces.iter().enumerate() {
			if idx < cur + p.len {
				return (i, idx - cur);
			}
			cur += p.len;
		}
		let last = self.pieces.len() - 1;
		(last, self.pieces[last].len)
	}

	fn rebuild_line_index(&mut self) {
		self.line_starts.clear();
		self.line_starts.push(0);
		let full = self.slice(0, self.len());
		for (i, c) in full.char_indices() {
			if c == '\n' {
				self.line_starts.push(i + 1);
			}
		}
	}

	fn update_line_index_on_insert(&mut self, idx: usize, text: &str) {
		let delta = text.len();

		let mut new_lines = Vec::new();
		for (i, c) in text.char_indices() {
			if c == '\n' {
				new_lines.push(idx + i + 1);
			}
		}

		let pos = self.line_starts.partition_point(|&off| off <= idx);

		self.line_starts.splice(pos..pos, new_lines.iter().cloned());

		let start_shift = (pos + new_lines.len()).max(1);
		for off in &mut self.line_starts[start_shift..] {
			*off += delta;
		}
	}

	fn update_line_index_on_delete(&mut self, start: usize, end: usize) {
		let delta = end - start;

		self.line_starts.retain(|&off| off < start || off >= end);

		let pos = self.line_starts.partition_point(|&off| off < end);

		let start_shift = pos.max(1);
		for off in &mut self.line_starts[start_shift..] {
			*off -= delta;
		}
	}
}

impl TextBuffer for PieceTable {
	fn len(&self) -> usize {
		self.pieces.iter().map(|p| p.len).sum()
	}

	fn char_at(&self, idx: usize) -> Option<char> {
		if idx >= self.len() {
			return None;
		}
		let (i, off) = self.locate(idx);
		self.buffer_slice(&self.pieces[i]).chars().nth(off)
	}

	fn slice(&self, start: usize, end: usize) -> String {
		assert!(start <= end && end <= self.len());
		let mut out = String::with_capacity(end - start);
		let mut cur = 0;
		for p in &self.pieces {
			let next = cur + p.len;
			if next <= start {
				cur = next;
				continue;
			}
			if cur >= end {
				break;
			}
			let s = start.max(cur) - cur;
			let e = end.min(next) - cur;
			self.push_bytes(p, s, e, &mut out);
			cur = next;
		}
		out
	}

	fn find(&self, pattern: &str, from: usize) -> Option<usize> {
		if pattern.is_empty() {
			return Some(from);
		}
		let limit = self.len().saturating_sub(pattern.len());
		let mut idx = from.min(limit);
		while idx <= limit {
			if &self.slice(idx, idx + pattern.len()) == pattern {
				return Some(idx);
			}
			idx += 1;
		}
		None
	}

	fn insert(&mut self, idx: usize, text: &str) {
		assert!(idx <= self.len());

		let (pi, off) = self.locate(idx);
		self.split_piece(pi, off);

		let add_start = self.add.len();
		self.add.push_str(text);

		let new_piece = Piece {
			source: BufferKind::Add,
			start:  add_start,
			len:    text.len(),
		};

		let at = if off == 0 { pi } else { pi + 1 };
		self.pieces.insert(at, new_piece);
		self.update_line_index_on_insert(idx, text);
	}

	fn delete(&mut self, start: usize, end: usize) {
		assert!(start < end && end <= self.len());

		let (si, soff) = self.locate(start);
		self.split_piece(si, soff);

		let (ei, eoff) = self.locate(end);
		self.split_piece(ei, eoff);

		let sidx = self.locate(start).0;
		let (eidx, eoff2) = self.locate(end);
		let to = eidx + if eoff2 > 0 { 1 } else { 0 };

		self.pieces.drain(sidx..to);
		self.update_line_index_on_delete(start, end);
	}

	fn rowcol_at(&self, idx: usize) -> RowCol {
		let row = self
			.line_starts
			.partition_point(|&off| off <= idx)
			.saturating_sub(1);

		RowCol {
			row,
			col: idx - self.line_starts[row],
		}
	}

	fn pos_from(&self, rc: RowCol) -> usize {
		self.line_starts
			.get(rc.row)
			.map(|&off| off + rc.col)
			.unwrap_or(self.len())
	}

	fn lines(&self) -> usize {
		self.line_starts.len()
	}

	fn line_len(&self, row: usize) -> usize {
		let start = *self.line_starts.get(row).unwrap_or(&self.len());
		let end = *self.line_starts.get(row + 1).unwrap_or(&self.len());
		self.slice(start, end).chars().count()
	}
}
