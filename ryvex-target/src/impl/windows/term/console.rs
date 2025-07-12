use super::{
	ffi,
	handle::{
		ConsoleHandle,
		ConsoleHandleSettings,
	},
};
use crate::std::{
	error::IoError,
	Result,
};
use crate::term::console::{
	Console,
	Handle,
};
use std::os::windows::prelude::RawHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsoleSettings {
	mode:   ffi::DWORD,
	cursor: ffi::CONSOLE_CURSOR_INFO,
}

impl ConsoleSettings {
	pub fn from_handle(handle: ConsoleHandle) -> Result<Self> {
		let mode = ffi::get_console_mode_from_handle(*handle.inner())
			.map_err(IoError::from)?;
		let cursor =
			ffi::get_cursor_info(*handle.inner()).map_err(IoError::from)?;

		Ok(Self { mode, cursor })
	}

	pub fn enter_raw_mode(&mut self, handle: ConsoleHandle) -> Result<Self> {
		let orig = *self;

		const RAW_MASK: ffi::DWORD =
			!(ffi::ENABLE_ECHO_INPUT | ffi::ENABLE_LINE_INPUT);

		let new_mode = self.mode & RAW_MASK;
		ffi::set_console_mode(*handle.inner(), new_mode)
			.map_err(IoError::from)?;

		self.mode = new_mode;
		Ok(orig)
	}

	fn inner(&self) -> &ffi::DWORD {
		&self.mode
	}

	fn inner_mut(&mut self) -> &mut ffi::DWORD {
		&mut self.mode
	}

	pub fn restore_terminal(
		handle: ConsoleHandle,
		orig: ConsoleSettings,
	) -> Result<()> {
		ffi::set_console_mode(*handle.inner(), orig.mode)
			.map_err(IoError::from)?;
		ffi::set_cursor_info(*handle.inner(), &orig.cursor)
			.map_err(IoError::from)?;
		Ok(())
	}
}

impl Console<RawHandle, ConsoleHandleSettings> for ConsoleSettings {
	type Handle = ConsoleHandle;

	fn init() -> Result<(Self, Self::Handle)> {
		let handle = ConsoleHandle::acquire(ConsoleHandleSettings::read())?;
		let termios = ConsoleSettings::from_handle(handle.clone())?;

		Ok((termios, handle))
	}

	fn raw(&mut self, handle: &Self::Handle) -> Result<Self> {
		self.enter_raw_mode(handle.clone())
	}

	fn restore(handle: &Self::Handle, orig: Self) -> Result<()> {
		ConsoleSettings::restore_terminal(handle.clone(), orig)
	}
}
