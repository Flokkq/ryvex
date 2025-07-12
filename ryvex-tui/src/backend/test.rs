use super::Backend;
use ryvex_target::std::Result;

pub struct TestBackend;

impl Backend for TestBackend {
	fn draw<'a, I>(&mut self, _content: I) -> Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
	{
		todo!("TestBackend")
	}

	fn show_cursor(
		&mut self,
		_kind: ryvex_ui::graphics::CursorKind,
	) -> Result<()> {
		todo!("TestBackend")
	}

	fn get_cursor(&mut self) -> Result<(u16, u16)> {
		todo!("TestBackend")
	}

	fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<()> {
		todo!("TestBackend")
	}

	fn clear(&mut self) -> Result<()> {
		todo!("TestBackend")
	}

	fn size(&self) -> Result<ryvex_ui::graphics::Rect> {
		todo!("TestBackend")
	}

	fn flush(&mut self) -> Result<()> {
		todo!("TestBackend")
	}

	fn hide_cursor(&mut self) -> Result<()> {
		todo!("TestBackend")
	}
}
