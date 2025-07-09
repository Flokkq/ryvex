use super::{
	ffi,
	handle::{
		ConsoleHandle,
		ConsoleHandleSettings,
	},
};
use crate::term::console::{
	Console,
	Handle,
};
use std::io;
use std::os::windows::prelude::RawHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsoleSettings {
	mode:   ffi::DWORD,
	cursor: ffi::CONSOLE_CURSOR_INFO,
}

impl ConsoleSettings {
	pub fn from_handle(handle: ConsoleHandle) -> io::Result<Self> {
		let mode = ffi::get_console_mode_from_handle(handle.inner().clone())?;
		let cursor = ffi::get_cursor_info(handle.inner().clone())?;

		Ok(Self { mode, cursor })
	}

	pub fn enter_raw_mode(
		&mut self,
		handle: ConsoleHandle,
	) -> io::Result<Self> {
		let orig = *self;

		const RAW_MASK: ffi::DWORD =
			!(ffi::ENABLE_ECHO_INPUT | ffi::ENABLE_LINE_INPUT);

		let new_mode = self.mode & RAW_MASK;
		ffi::set_console_mode(handle.inner().clone(), new_mode)?;

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
	) -> io::Result<()> {
		ffi::set_console_mode(handle.inner().clone(), orig.mode)?;
		ffi::set_cursor_info(handle.inner().clone(), &orig.cursor)?;
		Ok(())
	}
}

impl Console<RawHandle, ConsoleHandleSettings> for ConsoleSettings {
	type Handle = ConsoleHandle;

	fn init() -> Result<(Self, Self::Handle), io::Error> {
		let handle = ConsoleHandle::aquire(ConsoleHandleSettings::read())?;
		let termios = ConsoleSettings::from_handle(handle.clone())?;

		Ok((termios, handle))
	}

	fn raw(&mut self, handle: &Self::Handle) -> std::io::Result<Self> {
		self.enter_raw_mode(handle.clone())
	}

	fn restore(handle: &Self::Handle, orig: Self) -> std::io::Result<()> {
		ConsoleSettings::restore_terminal(handle.clone(), orig)
	}
}
