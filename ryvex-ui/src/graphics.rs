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
}
