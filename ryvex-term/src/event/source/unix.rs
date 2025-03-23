// File: ryvex-term/event/source/unix.rs

use std::{
	ffi::c_void,
	fs::OpenOptions,
	io,
	os::fd::IntoRawFd,
	time::Duration,
};

use crate::{
	error::Result,
	event::Event,
	key::AsciiKeyCode,
	sys::unix::{
		fd::TtyFd,
		ffi,
		target::STDIN_FILENO,
	},
};

use super::EventSource;

/// Unix event source implementation.
pub struct UnixEventSource {
	tty: TtyFd,
}

impl UnixEventSource {
	/// Creates a new UnixEventSource by obtaining a tty file descriptor.
	///
	/// If standard input is a tty, that fd is used (and not closed on drop).
	/// Otherwise, `/dev/tty` is opened (and will be closed on drop).
	pub fn new() -> Result<Self> {
		let (fd, close_on_drop) = unsafe {
			if ffi::isatty(STDIN_FILENO) == 1 {
				(STDIN_FILENO, false)
			} else {
				let file = OpenOptions::new()
					.read(true)
					.write(true)
					.open("/dev/tty")?;
				(file.into_raw_fd(), true)
			}
		};
		Ok(Self {
			tty: TtyFd::new(fd, close_on_drop),
		})
	}
}

impl EventSource for UnixEventSource {
	/// Attempts to read a single byte from the tty file descriptor.
	fn try_read(
		&mut self,
		_timeout: Option<Duration>,
	) -> Result<Option<Event>> {
		let fd = self.tty.fd();
		let mut buf = [0u8; 1];

		// Safety: We call our ffi::read, which is unsafe, but we pass a valid
		// pointer to a 1-byte buffer.
		let ret = unsafe { ffi::read(fd, buf.as_mut_ptr() as *mut c_void, 1) };
		if ret < 0 {
			let err = io::Error::last_os_error();
			if err.kind() == io::ErrorKind::WouldBlock {
				Ok(None)
			} else {
				Err(err.into())
			}
		} else if ret == 0 {
			// No data read (or end-of-file).
			Ok(None)
		} else {
			// Convert the byte into an AsciiKeyCode and wrap it as a Key event.
			let key = AsciiKeyCode::from_ascii(buf[0]);
			Ok(Some(Event::Key(key)))
		}
	}
}
