use super::{
	get_console_mode_from_handle,
	set_console_mode,
};
use crate::{
	error::Result,
	sys::windows::{
		fd::TtyFd,
		ffi::{
			DWORD,
			ENABLE_ECHO_INPUT,
			ENABLE_LINE_INPUT,
			ENABLE_PROCESSED_INPUT,
		},
		ConsoleHandle,
	},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Termios {
	mode: DWORD,
}

impl Termios {
	pub fn from_handle(handle: ConsoleHandle) -> Result<Self> {
		let mode = unsafe { get_console_mode_from_handle(handle.as_raw())? };
		Ok(Self { mode })
	}

	pub fn raw(&mut self, handle: ConsoleHandle) -> Result<Self> {
		let orig = *self;

		const RAW_MASK: DWORD =
			!(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT);

		let new_mode = self.mode & RAW_MASK;
		unsafe { set_console_mode(handle.as_raw(), new_mode)? };

		self.mode = new_mode;
		Ok(orig)
	}

	fn inner(&self) -> &DWORD {
		&self.mode
	}

	fn inner_mut(&mut self) -> &mut DWORD {
		&mut self.mode
	}

	pub fn restore_terminal(
		handle: ConsoleHandle,
		orig: Termios,
	) -> Result<()> {
		unsafe { Ok(set_console_mode(handle.as_raw(), orig.mode)?) }
	}
}
