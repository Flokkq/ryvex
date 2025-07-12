use crate::std::error::IoError;
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
use core::fmt::Display;

use super::{
	cursor,
	terminal,
};

impl ExecuteApi for MoveTo {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_to(
			self.1.saturating_sub(1) as i16,
			self.0.saturating_sub(1) as i16,
		)
		.map_err(IoError::from)
	}
}

impl ExecuteApi for MoveToNextLine {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_to_next_line(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveToPreviousLine {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_to_previous_line(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveToColumn {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_to_column(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveToRow {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_to_row(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveUp {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_up(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveDown {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_down(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveLeft {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_left(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for MoveRight {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::move_right(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for SavePosition {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::save_position().map_err(IoError::from)
	}
}

impl ExecuteApi for RestorePosition {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::restore_position().map_err(IoError::from)
	}
}

impl ExecuteApi for Show {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::show_cursor(true).map_err(IoError::from)
	}
}

impl ExecuteApi for Hide {
	fn execute_api(&self) -> Result<(), IoError> {
		cursor::show_cursor(false).map_err(IoError::from)
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
	fn execute_api(&self) -> Result<(), IoError> {
		terminal::scroll_up(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for ScrollDown {
	fn execute_api(&self) -> Result<(), IoError> {
		terminal::scroll_down(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for Clear {
	fn execute_api(&self) -> Result<(), IoError> {
		terminal::clear(self.0).map_err(IoError::from)
	}
}

impl ExecuteApi for SetSize {
	fn execute_api(&self) -> Result<(), IoError> {
		terminal::set_size(self.0, self.1).map_err(IoError::from)
	}
}

impl<T: Display> ExecuteApi for Print<T> {
	fn execute_api(&self) -> Result<(), IoError> {
		terminal::write(&self.0.to_string()).map_err(IoError::from)
	}

	fn is_ansi_code_supported(&self) -> bool {
		true
	}
}
