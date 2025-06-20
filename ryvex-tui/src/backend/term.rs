use super::Backend;
use ryvex_term::sys::unix::fd::TtyFd;
use ryvex_ui::graphics::CursorKind;
use std::io::Write;

pub struct TerminalBackend {
	buffer: std::io::Stdout,
	fd:     TtyFd,
}

impl TerminalBackend {
	pub fn new(fd: TtyFd) -> Self {
		Self {
			fd,
			buffer: std::io::stdout(),
		}
	}
}

impl std::io::Write for TerminalBackend {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.buffer.write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
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
				let _ = write!(self.buffer, "\x1b[{};{}H", y + 1, x + 1);
			}
			last_pos = Some((x, y));

			let _ = write!(self.buffer, "{}", cell.to_string());
		}

		Ok(())
	}

	fn show_cursor(
		&mut self,
		kind: ryvex_ui::graphics::CursorKind,
	) -> super::Result<()> {
		write!(self.buffer, "\x1b[?25h")?;

		match kind {
			CursorKind::Block => write!(self.buffer, "\x1b[0 q")?,
			CursorKind::Underline => write!(self.buffer, "\x1b[4 q")?,
			CursorKind::Bar => write!(self.buffer, "\x1b[6 q")?,
		}

		Ok(())
	}

	fn get_cursor(&mut self) -> super::Result<(u16, u16)> {
		todo!("TerminalBackend")
	}

	fn set_cursor(&mut self, x: u16, y: u16) -> super::Result<()> {
		todo!("TerminalBackend")
	}

	fn clear(&mut self) -> super::Result<()> {
		Ok(write!(self.buffer, "\x1b[2J")?)
	}

	fn size(&self) -> super::Result<ryvex_ui::graphics::Rect> {
		Ok(ryvex_term::get_terminal_size(&self.fd)?)
	}

	fn flush(&mut self) -> super::Result<()> {
		Ok(self.buffer.flush()?)
	}

	fn hide_cursor(&mut self) -> super::Result<()> {
		Ok(write!(self.buffer, "\x1b[?25l")?)
	}
}
