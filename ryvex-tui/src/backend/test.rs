use super::Backend;

pub struct TestBackend;

impl Backend for TestBackend {
	fn draw<'a, I>(&mut self, content: I) -> super::Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
	{
		println!("Test");
		todo!("TestBackend")
	}

	fn show_cursor(
		&mut self,
		kind: ryvex_ui::graphics::CursorKind,
	) -> super::Result<()> {
		todo!("TestBackend")
	}

	fn get_cursor(&mut self) -> super::Result<(u16, u16)> {
		todo!("TestBackend")
	}

	fn set_cursor(&mut self, x: u16, y: u16) -> super::Result<()> {
		todo!("TestBackend")
	}

	fn clear(&mut self) -> super::Result<()> {
		todo!("TestBackend")
	}

	fn size(&self) -> super::Result<ryvex_ui::graphics::Rect> {
		todo!("TestBackend")
	}

	fn flush(&mut self) -> super::Result<()> {
		todo!("TestBackend")
	}

	fn hide_cursor(&mut self) -> super::Result<()> {
		todo!("TestBackend")
	}
}
