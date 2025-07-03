use core::fmt;

use crate::{
	command::Command,
	csi,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScrollUp(pub u16);

impl Command for ScrollUp {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		if self.0 != 0 {
			write!(f, csi!("{}S"), self.0)?;
		}
		Ok(())
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> std::io::Result<()> {
		use crate::sys::windows;

		windows::scroll_up(self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScrollDown(pub u16);

impl Command for ScrollDown {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		if self.0 != 0 {
			write!(f, csi!("{}T"), self.0)?;
		}
		Ok(())
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> std::io::Result<()> {
		use crate::sys::windows;

		windows::scroll_down(self.0)
	}
}
