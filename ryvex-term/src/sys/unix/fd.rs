use std::{
	fs::OpenOptions,
	os::fd::{
		IntoRawFd,
		RawFd,
	},
};

use super::{
	ffi,
	target::STDIN_FILENO,
};

use crate::error::Result;

// RAII wrapper for a file descriptor.
/// If `close_on_drop` is true, the descriptor will be closed on drop.
pub struct TtyFd {
	fd:            RawFd,
	close_on_drop: bool,
}

impl TtyFd {
	pub fn new(fd: RawFd, close_on_drop: bool) -> Self {
		Self { fd, close_on_drop }
	}

	fn from_default_tty(read: bool, write: bool) -> Result<Self> {
		let (fd, close_on_drop) = if is_tty(STDIN_FILENO) {
			(STDIN_FILENO, false)
		} else {
			let file = OpenOptions::new()
				.read(read)
				.write(write)
				.open("/dev/tty")?;
			(file.into_raw_fd(), true)
		};

		Ok(Self { fd, close_on_drop })
	}

	pub fn read() -> Result<Self> {
		Self::from_default_tty(true, false)
	}

	pub fn write() -> Result<Self> {
		Self::from_default_tty(false, true)
	}

	/// Returns the underlying RawFd.
	pub fn fd(&self) -> RawFd {
		self.fd
	}
}

impl Drop for TtyFd {
	fn drop(&mut self) {
		if self.close_on_drop {
			// Safety: `close` is only called on a valid file descriptor
			// that we know should be closed.
			unsafe {
				ffi::close(self.fd);
			}
		}
	}
}

fn is_tty(fd: RawFd) -> bool {
	unsafe { ffi::isatty(fd) == 1 }
}
