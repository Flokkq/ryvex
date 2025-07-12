use super::Backend;
use ryvex_target::{
	execute,
	queue,
	r#impl::TargetOutWriter,
	std::{
		error::IoError,
		write::Write,
	},
	target::{
		self,
		term::Handle,
	},
	term::command::{
		cursor::{
			Hide,
			MoveTo,
			SetCursorStyle,
			Show,
		},
		terminal::{
			Clear,
			ClearType,
			Print,
		},
	},
};
use ryvex_ui::graphics::CursorKind;

pub struct TerminalBackend {
	buffer: TargetOutWriter,
	fd:     Handle,
}

impl TerminalBackend {
	pub fn new(fd: Handle) -> Self {
		Self {
			fd,
			buffer: TargetOutWriter::default(),
		}
	}
}

impl Write for TerminalBackend {
	fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
		self.buffer.write(buf)
	}

	fn flush(&mut self) -> Result<(), IoError> {
		self.buffer.flush()
	}
}

impl Backend for TerminalBackend {
	fn draw<'a, I>(&mut self, content: I) -> super::Result<()>
	where
		I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
	{
		let mut last_pos: Option<(u16, u16)> = None;
		for (x, y, cell) in content {
			if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
				execute!(self.buffer, MoveTo(y + 1, x + 1))?;
			}
			last_pos = Some((x, y));

			queue!(self.buffer, Print(&cell.symbol))?;
		}

		Ok(())
	}

	fn show_cursor(
		&mut self,
		kind: ryvex_ui::graphics::CursorKind,
	) -> super::Result<()> {
		execute!(self.buffer, Show)?;

		match kind {
			CursorKind::Block => {
				execute!(self.buffer, SetCursorStyle::SteadyBlock)?
			}
			CursorKind::Underline => {
				execute!(self.buffer, SetCursorStyle::SteadyUnderScore)?
			}
			CursorKind::Bar => {
				execute!(self.buffer, SetCursorStyle::SteadyBar)?
			}
		}

		Ok(())
	}

	fn get_cursor(&mut self) -> super::Result<(u16, u16)> {
		todo!("TerminalBackend")
	}

	fn set_cursor(&mut self, x: u16, y: u16) -> super::Result<()> {
		Ok(execute!(self.buffer, MoveTo(x, y))?)
	}

	fn clear(&mut self) -> super::Result<()> {
		Ok(execute!(self.buffer, Clear(ClearType::All))?)
	}

	fn size(&self) -> super::Result<ryvex_ui::graphics::Rect> {
		target::term::get_terminal_size(&self.fd)
	}

	fn flush(&mut self) -> super::Result<()> {
		Ok(self.buffer.flush()?)
	}

	fn hide_cursor(&mut self) -> super::Result<()> {
		Ok(execute!(self.buffer, Hide)?)
	}
}
