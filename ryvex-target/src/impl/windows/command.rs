use std::fmt::Display;

use crate::term::command::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		Hide,
		MoveDown,
		MoveLeft,
		MoveRight,
		MoveTo,
		MoveToColumn,
		MoveToNextLine,
		MoveToPreviousLine,
		MoveToRow,
		MoveUp,
		RestorePosition,
		SavePosition,
		SetCursorStyle,
		Show,
	},
	terminal::{
		Clear,
		Print,
		ScrollDown,
		ScrollUp,
		SetSize,
	},
	ExecuteApi,
};

use super::{
	cursor,
	terminal,
};
use std::io;

impl ExecuteApi for MoveTo {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_to(
			self.1.saturating_sub(1) as i16,
			self.0.saturating_sub(1) as i16,
		)
	}
}

impl ExecuteApi for MoveToNextLine {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_to_next_line(self.0)
	}
}

impl ExecuteApi for MoveToPreviousLine {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_to_previous_line(self.0)
	}
}

impl ExecuteApi for MoveToColumn {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_to_column(self.0)
	}
}

impl ExecuteApi for MoveToRow {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_to_row(self.0)
	}
}

impl ExecuteApi for MoveUp {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_up(self.0)
	}
}

impl ExecuteApi for MoveDown {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_down(self.0)
	}
}

impl ExecuteApi for MoveLeft {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_left(self.0)
	}
}

impl ExecuteApi for MoveRight {
	fn execute_api(&self) -> io::Result<()> {
		cursor::move_right(self.0)
	}
}

impl ExecuteApi for SavePosition {
	fn execute_api(&self) -> io::Result<()> {
		cursor::save_position()
	}
}

impl ExecuteApi for RestorePosition {
	fn execute_api(&self) -> io::Result<()> {
		cursor::restore_position()
	}
}

impl ExecuteApi for Show {
	fn execute_api(&self) -> std::io::Result<()> {
		cursor::show_cursor(true)
	}
}

impl ExecuteApi for Hide {
	fn execute_api(&self) -> std::io::Result<()> {
		cursor::show_cursor(false)
	}
}

impl ExecuteApi for EnableBlinking {
	fn is_ansi_code_supported(&self) -> bool {
		false
	}
}

impl ExecuteApi for DisableBlinking {
	fn is_ansi_code_supported(&self) -> bool {
		false
	}
}

impl ExecuteApi for SetCursorStyle {
	fn is_ansi_code_supported(&self) -> bool {
		false
	}
}

impl ExecuteApi for ScrollUp {
	fn execute_api(&self) -> std::io::Result<()> {
		terminal::scroll_up(self.0)
	}
}

impl ExecuteApi for ScrollDown {
	fn execute_api(&self) -> std::io::Result<()> {
		terminal::scroll_down(self.0)
	}
}

impl ExecuteApi for Clear {
	fn execute_api(&self) -> std::io::Result<()> {
		terminal::clear(self.0)
	}
}

impl ExecuteApi for SetSize {
	fn execute_api(&self) -> std::io::Result<()> {
		terminal::set_size(self.0, self.1)
	}
}

impl<T: Display> ExecuteApi for Print<T> {
	fn execute_api(&self) -> std::io::Result<()> {
		terminal::write(&self.0.to_string())
	}

	fn is_ansi_code_supported(&self) -> bool {
		true
	}
}
