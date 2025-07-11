pub mod term;
#[cfg(test)]
pub mod test;

use ryvex_target::std::Result;
use ryvex_ui::graphics::{
	CursorKind,
	Rect,
};

use crate::buffer::Cell;

pub trait Backend {
	fn draw<'a, I>(&mut self, content: I) -> Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a Cell)>;
	fn hide_cursor(&mut self) -> Result<()>;
	fn show_cursor(&mut self, kind: CursorKind) -> Result<()>;
	fn get_cursor(&mut self) -> Result<(u16, u16)>;
	fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;
	fn clear(&mut self) -> Result<()>;
	fn size(&self) -> Result<Rect>;
	fn flush(&mut self) -> Result<()>;
}
