use std::os::fd::RawFd;

use super::ffi;

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
