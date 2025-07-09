use core::fmt;
use std::fmt::Display;

use crate::csi;

use super::WriteAnsi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollUp(pub u16);

impl WriteAnsi for ScrollUp {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		if self.0 != 0 {
			write!(f, csi!("{}S"), self.0)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollDown(pub u16);

impl WriteAnsi for ScrollDown {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		if self.0 != 0 {
			write!(f, csi!("{}T"), self.0)?;
		}
		Ok(())
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

impl WriteAnsi for Clear {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetSize(pub u16, pub u16);

impl WriteAnsi for SetSize {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		write!(f, csi!("8;{};{}t"), self.1, self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Print<T: Display>(pub T);

impl<T: Display> WriteAnsi for Print<T> {
	fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl<T: Display> Display for Print<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}
