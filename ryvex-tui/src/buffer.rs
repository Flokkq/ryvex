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
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Cell {
	pub symbol: String,
}
