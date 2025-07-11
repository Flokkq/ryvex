use super::ffi;
use std::{
	os::windows::io::RawHandle,
	ptr,
};

use crate::{
	c,
	std::{
		error::IoError,
		Result,
	},
	term::console::Handle,
};

pub struct ConsoleHandleSettings {
	read:  bool,
	write: bool,
}

impl ConsoleHandleSettings {
	pub fn read() -> Self {
		Self {
			read:  true,
			write: false,
		}
	}

	pub fn write() -> Self {
		Self {
			read:  false,
			write: true,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConsoleHandle {
	handle:        RawHandle,
	close_on_drop: bool,
}

impl ConsoleHandle {
	pub fn from_default_tty(read: bool, _write: bool) -> Result<Self> {
		let handle = ffi::get_current_out_handle();
		if let Ok(h) = handle {
			return Ok(ConsoleHandle {
				handle:        h as RawHandle,
				close_on_drop: false,
			});
		}

		let (name, desired) = if read {
			(c!("CONIN$"), ffi::GENERIC_READ)
		} else {
			(c!("CONOUT$"), ffi::GENERIC_WRITE)
		};

		let handle = ffi::create_file_a(
			name,
			desired,
			ffi::FILE_SHARE_READ | ffi::FILE_SHARE_WRITE,
			ptr::null_mut(),
			ffi::OPEN_EXISTING,
			0,
			ptr::null_mut(),
		)
		.map_err(IoError::from)?;

		Ok(ConsoleHandle {
			handle:        handle as RawHandle,
			close_on_drop: true,
		})
	}
}

impl Handle<RawHandle, ConsoleHandleSettings> for ConsoleHandle {
	fn acquire(mode: ConsoleHandleSettings) -> Result<Self> {
		ConsoleHandle::from_default_tty(mode.read, mode.write)
	}

	fn inner(&self) -> &RawHandle {
		&self.handle
	}

	fn inner_mut(&mut self) -> &mut RawHandle {
		&mut self.handle
	}
}

impl Drop for ConsoleHandle {
	fn drop(&mut self) {
		if self.close_on_drop {
			let _ = ffi::close_handle(self.handle);
		}
	}
}
