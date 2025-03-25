use ryvex_ui::rect::Rect;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffer {
	pub area:    Rect,
	pub content: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
	pub symbol: String,
}
