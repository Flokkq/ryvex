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

	pub fn area(&self) -> usize {
		(self.width as usize) * (self.height as usize)
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
