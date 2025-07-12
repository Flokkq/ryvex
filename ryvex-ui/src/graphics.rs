#[derive(Debug, Default, Clone, Copy, Hash, PartialEq)]
pub struct Rect {
	pub x:      u16,
	pub y:      u16,
	pub width:  u16,
	pub height: u16,
}

impl Rect {
	pub fn new() -> Self {
		Self::default()
	}

	#[inline]
	pub fn area(&self) -> usize {
		(self.width as usize) * (self.height as usize)
	}

	#[inline]
	pub fn left(self) -> u16 {
		self.x
	}

	#[inline]
	pub fn right(self) -> u16 {
		self.x.saturating_add(self.width)
	}

	#[inline]
	pub fn top(self) -> u16 {
		self.y
	}

	#[inline]
	pub fn bottom(self) -> u16 {
		self.y.saturating_add(self.height)
	}

	pub fn clip_left(self, width: u16) -> Rect {
		let width = core::cmp::min(width, self.width);
		Rect {
			x: self.x.saturating_add(width),
			width: self.width.saturating_sub(width),
			..self
		}
	}

	pub fn clip_right(self, width: u16) -> Rect {
		Rect {
			width: self.width.saturating_sub(width),
			..self
		}
	}

	pub fn clip_top(self, height: u16) -> Rect {
		let height = core::cmp::min(height, self.height);
		Rect {
			y: self.y.saturating_add(height),
			height: self.height.saturating_sub(height),
			..self
		}
	}

	pub fn clip_bottom(self, height: u16) -> Rect {
		Rect {
			height: self.height.saturating_sub(height),
			..self
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorKind {
	/// █
	Block,
	/// |
	Bar,
	/// _
	Underline,
}
