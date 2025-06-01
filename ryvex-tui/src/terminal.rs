use ryvex_ui::graphics::{
	CursorKind,
	Rect,
};

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
		self.flush()?;

		if let Some((x, y)) = cursor_position {}

		match cursor_kind {
			CursorKind::Block => {}
			CursorKind::Bar => {}
			CursorKind::Underline => {}
		}

		self.buffers[1 - self.current].reset();
		self.current = 1 - self.current;

		self.backend.flush()?;

		Ok(())
	}

	pub fn flush(&mut self) -> Result<()> {
		self.backend.flush()?;
		Ok(self.backend.draw(
			self.buffers[self.current].content.iter().enumerate().map(
				|(i, cell)| {
					let x = (i as u16) % self.buffers[self.current].area.width;
					let y = (i as u16) / self.buffers[self.current].area.width;
					(x, y, cell)
				},
			),
		)?)
	}

	pub fn clear(&mut self) -> Result<()> {
		self.backend.clear()?;
		self.buffers[1 - self.current].reset();
		Ok(())
	}

	pub fn current_buffer_mut(&mut self) -> &mut Buffer {
		&mut self.buffers[self.current]
	}

	pub fn size(&self) -> Result<Rect> {
		Ok(self.backend.size()?)
	}
}
