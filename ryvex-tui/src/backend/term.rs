use super::Backend;

pub struct TerminalBackend;

impl TerminalBackend {
	pub fn new() -> Self {
		Self {}
	}
}

impl Backend for TerminalBackend {
	fn draw<'a, I>(&mut self, content: I) -> super::Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
	{
		todo!("TerminalBackend")
	}

	fn show_cursor(
		&mut self,
		kind: ryvex_ui::graphics::CursorKind,
	) -> super::Result<()> {
		todo!("TerminalBackend")
	}

	fn get_cursor(&mut self) -> super::Result<(u16, u16)> {
		todo!("TerminalBackend")
	}

	fn set_cursor(&mut self, x: u16, y: u16) -> super::Result<()> {
		todo!("TerminalBackend")
	}

	fn clear(&mut self) -> super::Result<()> {
		todo!("TerminalBackend")
	}

	fn size(&self) -> super::Result<ryvex_ui::graphics::Rect> {
		todo!("TerminalBackend")
	}

	fn flush(&mut self) -> super::Result<()> {
		todo!("TerminalBackend")
	}

	fn hide_cursor(&mut self) -> super::Result<()> {
		todo!("TerminalBackend")
	}
}
