use ryvex_ui::graphics::CursorKind;

use crate::{
	backend::Backend,
	buffer::Buffer,
	error::Result,
};

pub struct Terminal<B: Backend> {
	backend: B,

	buffers:     [Buffer; 2],
	current:     usize,
	cursor_kind: CursorKind,
}

impl<B> Terminal<B>
where
	B: Backend,
{
	pub fn new(backend: B) -> Result<Self> {
		let size = backend.size()?;

		Ok(Self {
			backend,
			buffers: [Buffer::empty(size), Buffer::empty(size)],
			current: 0,
			cursor_kind: CursorKind::Block,
		})
	}

	pub fn draw(
		&mut self,
		cursor_position: Option<(u16, u16)>,
		cursor_kind: CursorKind,
	) -> Result<()> {
		Ok(())
	}
}
