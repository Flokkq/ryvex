use ryvex_ui::graphics::{
	CursorKind,
	Rect,
};

use crate::{
	backend::Backend,
	buffer::Buffer,
};

use ryvex_target::std::Result;

pub struct Terminal<B: Backend> {
	backend: B,

	buffers:      [Buffer; 2],
	current:      usize,
	_cursor_kind: CursorKind,
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
			_cursor_kind: CursorKind::Block,
		})
	}

	pub fn draw(
		&mut self,
		cursor_position: Option<(u16, u16)>,
		cursor_kind: CursorKind,
	) -> Result<()> {
		self.backend.hide_cursor()?;
		self.flush()?;

		if let Some((x, y)) = cursor_position {
			self.backend.set_cursor(x, y)?;
		}
		self.backend.show_cursor(cursor_kind)?;

		self.buffers[1 - self.current].reset();
		self.current = 1 - self.current;

		self.backend.flush()?;

		Ok(())
	}

	pub fn flush(&mut self) -> Result<()> {
		let previous_buffer = &self.buffers[1 - self.current];
		let current_buffer = &self.buffers[self.current];
		let updates = previous_buffer.diff(current_buffer);

		self.backend.draw(updates.into_iter())
	}

	pub fn clear(&mut self) -> Result<()> {
		self.backend.clear()?;
		self.buffers.iter_mut().for_each(Buffer::reset);
		Ok(())
	}

	pub fn current_buffer_mut(&mut self) -> &mut Buffer {
		&mut self.buffers[self.current]
	}

	pub fn size(&self) -> Result<Rect> {
		self.backend.size()
	}
}
