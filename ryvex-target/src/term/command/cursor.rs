use crate::csi;

use super::WriteAnsi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo(pub u16, pub u16);

impl WriteAnsi for MoveTo {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{};{}H"), self.0, self.1)
	}
}

/// This command is one based, meaning `MoveToNextLine(1)` move to the next
/// line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToNextLine(pub u16);

impl WriteAnsi for MoveToNextLine {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}E"), self.0)
	}
}

/// This command is one based, meaning `MoveToPreviousLine(1)` move to the next
/// line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToPreviousLine(pub u16);

impl WriteAnsi for MoveToPreviousLine {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}F"), self.0)
	}
}

/// This command is zero based, meaning `MoveToColumn(0)` moves to the leftmost
/// column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToColumn(pub u16);

impl WriteAnsi for MoveToColumn {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}G"), self.0 + 1)
	}
}

/// This command is zero based, meaning `MoveToRow(0)` is the topmost column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToRow(pub u16);

impl WriteAnsi for MoveToRow {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}d"), self.0 + 1)
	}
}

/// This command is one based, meaning `MoveUp(1)` moves the cursor up one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);

impl WriteAnsi for MoveUp {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}A"), self.0)
	}
}

/// This command is one based, meaning `MoveDown(1)` moves the cursor down one
/// cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);

impl WriteAnsi for MoveDown {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}B"), self.0)
	}
}

/// This command is one based, meaning `MoveLeft(1)` moves the cursor left one
/// cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);

impl WriteAnsi for MoveLeft {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}D"), self.0)
	}
}

/// This command is one based, meaning `MoveRight(1)` moves the cursor right one
/// cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);

impl WriteAnsi for MoveRight {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("{}C"), self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavePosition;

impl WriteAnsi for SavePosition {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("s"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestorePosition;

impl WriteAnsi for RestorePosition {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		write!(f, csi!("u"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Show;

impl WriteAnsi for Show {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		f.write_str(csi!("?25h"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hide;

impl WriteAnsi for Hide {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		f.write_str(csi!("?25l"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBlinking;

impl WriteAnsi for EnableBlinking {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		f.write_str(csi!("?12h"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBlinking;

impl WriteAnsi for DisableBlinking {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		f.write_str(csi!("?12l"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetCursorStyle {
	/// Default cursor shape configured by the user.
	DefaultUserShape,
	/// A blinking block cursor shape (■).
	BlinkingBlock,
	/// A non blinking block cursor shape (inverse of `BlinkingBlock`).
	SteadyBlock,
	/// A blinking underscore cursor shape(_).
	BlinkingUnderScore,
	/// A non blinking underscore cursor shape (inverse of
	/// `BlinkingUnderScore`).
	SteadyUnderScore,
	/// A blinking cursor bar shape (|)
	BlinkingBar,
	/// A steady cursor bar shape (inverse of `BlinkingBar`).
	SteadyBar,
}

impl WriteAnsi for SetCursorStyle {
	fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
		match self {
			SetCursorStyle::DefaultUserShape => f.write_str("\x1b[0 q"),
			SetCursorStyle::BlinkingBlock => f.write_str("\x1b[1 q"),
			SetCursorStyle::SteadyBlock => f.write_str("\x1b[2 q"),
			SetCursorStyle::BlinkingUnderScore => f.write_str("\x1b[3 q"),
			SetCursorStyle::SteadyUnderScore => f.write_str("\x1b[4 q"),
			SetCursorStyle::BlinkingBar => f.write_str("\x1b[5 q"),
			SetCursorStyle::SteadyBar => f.write_str("\x1b[6 q"),
		}
	}
}
