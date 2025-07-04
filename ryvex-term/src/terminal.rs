use core::fmt;
use std::fmt::Display;

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

/// Different ways to clear the terminal buffer.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ClearType {
	/// All cells.
	All,
	/// All plus history
	Purge,
	/// All cells from the cursor position downwards.
	FromCursorDown,
	/// All cells from the cursor position upwards.
	FromCursorUp,
	/// All cells at the cursor row.
	CurrentLine,
	/// All cells from the cursor position until the new line.
	UntilNewLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub ClearType);

impl Command for Clear {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		f.write_str(match self.0 {
			ClearType::All => csi!("2J"),
			ClearType::Purge => csi!("3J"),
			ClearType::FromCursorDown => csi!("J"),
			ClearType::FromCursorUp => csi!("1J"),
			ClearType::CurrentLine => csi!("2K"),
			ClearType::UntilNewLine => csi!("K"),
		})
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> std::io::Result<()> {
		use crate::sys::windows;

		windows::clear(self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetSize(pub u16, pub u16);

impl Command for SetSize {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		write!(f, csi!("8;{};{}t"), self.1, self.0)
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> std::io::Result<()> {
		use crate::sys::windows;

		windows::set_size(self.0, self.1)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Print<T: Display>(pub T);

impl<T: Display> Command for Print<T> {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		write!(f, "{}", self.0)
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> std::io::Result<()> {
		use crate::sys::windows;

		windows::write(&self.0.to_string())
	}

	#[cfg(windows)]
	fn is_ansi_code_supported(&self) -> bool {
		true
	}
}

impl<T: Display> Display for Print<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}
