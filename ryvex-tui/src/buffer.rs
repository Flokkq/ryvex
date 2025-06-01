use std::fmt::Display;

use ryvex_ui::graphics::Rect;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffer {
	pub area:    Rect,
	pub content: Vec<Cell>,
}

impl Buffer {
	pub fn empty(area: Rect) -> Buffer {
		Buffer::filled(area, &Cell::default())
	}

	pub fn filled(area: Rect, cell: &Cell) -> Buffer {
		let size = area.area();
		let content = vec![cell.clone(); size];
		Buffer { area, content }
	}

	pub fn reset(&mut self) {
		for cell in &mut self.content {
			cell.reset();
		}
	}

	pub fn set_string<S>(&mut self, x: u16, y: u16, string: S)
	where
		S: AsRef<str>,
	{
		if !self.in_bounds(x, y) {
			return;
		}

		let mut idx = self.index_of(x, y);
		for c in string.as_ref().chars() {
			self.content[idx].set_symbol(&c.to_string());

			if idx + 1 > self.content.len() {
				break;
			}

			idx += 1;
		}
	}

	pub fn in_bounds(&self, x: u16, y: u16) -> bool {
		x >= self.area.left() &&
			x < self.area.right() &&
			y >= self.area.top() &&
			y < self.area.bottom()
	}

	pub fn index_of(&self, x: u16, y: u16) -> usize {
		debug_assert!(
			self.in_bounds(x, y),
			"Trying to access position outside the buffer: x={}, y={}, \
			 area={:?}",
			x,
			y,
			self.area
		);
		((y - self.area.y) as usize) * (self.area.width as usize) +
			((x - self.area.x) as usize)
	}
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Cell {
	pub symbol: String,
}

impl Cell {
	pub fn reset(&mut self) {
		self.symbol.clear();
		self.symbol.push(' ');
	}

	pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
		self.symbol.clear();
		self.symbol.push_str(symbol);
		self
	}
}

impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.symbol)
	}
}
