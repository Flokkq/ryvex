use crate::{
	command::Command,
	csi,
};
use std::io;

pub struct MoveCursor(pub u16, pub u16);

impl Command for MoveCursor {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		write!(f, csi!("{};{}H"), self.0, self.1)
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> io::Result<()> {
		use crate::sys::windows;

		windows::set_cursor_position(self.0, self.1)
	}
}
