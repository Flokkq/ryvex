use crate::c;
use crate::sys::windows::close_handle;
use crate::sys::windows::get_current_out_handle;
use crate::sys::windows::open_device;
use std::{
	ffi::c_void,
	mem::MaybeUninit,
	os::windows::io::{
		IntoRawHandle,
		RawHandle,
	},
	ptr,
};

use super::ffi;
use crate::error::{
	Result,
	TermError,
};

pub struct TtyFd {
	handle:        RawHandle,
	close_on_drop: bool,
}

impl TtyFd {
	pub fn new(handle: RawHandle, close_on_drop: bool) -> Self {
		Self {
			handle,
			close_on_drop,
		}
	}

	fn from_default_tty(read: bool, write: bool) -> Result<Self> {
		let handle = unsafe { get_current_out_handle() };
		match handle {
			Ok(h) => return Ok(Self::new(h as RawHandle, false)),
			_ => {}
		};

		let (name, desired) = if read {
			(c!("CONIN$"), ffi::GENERIC_READ)
		} else {
			(c!("CONOUT$"), ffi::GENERIC_WRITE)
		};

		let handle = unsafe { open_device(name, desired)? };

		Ok(Self::new(handle as RawHandle, true))
	}

	pub fn read() -> Result<Self> {
		Self::from_default_tty(true, false)
	}
	pub fn write() -> Result<Self> {
		Self::from_default_tty(false, true)
	}

	pub fn handle(&self) -> RawHandle {
		self.handle
	}
}

impl Drop for TtyFd {
	fn drop(&mut self) {
		if self.close_on_drop {
			unsafe {
				let _ = close_handle(self.handle);
			};
		}
	}
}
