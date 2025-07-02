use crate::{
	command::Command,
	csi,
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo(pub u16, pub u16);

impl Command for MoveTo {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		write!(f, csi!("{};{}H"), self.0, self.1)
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> io::Result<()> {
		use crate::sys::windows;

		windows::move_to(self.0 as i16, self.1 as i16)
	}
}

/// This command is one based, meaning `MoveToNextLine(1)` move to the next line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToNextLine(pub u16);

impl Command for MoveToNextLine {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		write!(f, csi!("{}E"), self.0)
	}

	#[cfg(windows)]
	fn execute_winapi(&self) -> io::Result<()> {
		use crate::sys::windows;

		windows::move_to_next_line(self.0)
	}
}


/// This command is one based, meaning `MoveToPreviousLine(1)` move to the next line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToPreviousLine(pub u16);

impl Command for MoveToPreviousLine {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}F"), self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_to_previous_line(self.0)
    }
}

/// This command is zero based, meaning `MoveToColumn(0)` moves to the leftmost column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToColumn(pub u16);

impl Command for MoveToColumn {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}G"), self.0 + 1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_to_column(self.0)
    }
}

/// This command is zero based, meaning `MoveToRow(0)` is the topmost column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToRow(pub u16);

impl Command for MoveToRow {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}d"), self.0 + 1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_to_row(self.0)
    }
}

/// This command is one based, meaning `MoveUp(1)` moves the cursor up one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);


impl Command for MoveUp {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}A"), self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_up(self.0)
    }
}


/// This command is one based, meaning `MoveDown(1)` moves the cursor down one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);


impl Command for MoveDown {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}B"), self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_down(self.0)
    }
}


/// This command is one based, meaning `MoveLeft(1)` moves the cursor left one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);


impl Command for MoveLeft {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}D"), self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_left(self.0)
    }
}


/// This command is one based, meaning `MoveRight(1)` moves the cursor right one cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);


impl Command for MoveRight {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("{}C"), self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::move_right(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavePosition;


impl Command for SavePosition {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("\x1B7"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::save_position()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestorePosition;


impl Command for RestorePosition {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("\x1B8"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        use crate::sys::windows;

        windows::restore_position()
    }
}
