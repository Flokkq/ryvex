// File: ryvex-term/event/source/unix.rs

use std::{
	error::Error,
	io,
	os::fd::RawFd,
	time::Duration,
};

use crate::{
	error::{
		Result,
		TermError,
	},
	event::Event,
	key::AsciiKeyCode,
	sys::unix::{
		fd::TtyFd,
		ffi,
	},
};

use super::EventSource;

/// Unix event source implementation.
pub struct UnixEventSource {
	tty: TtyFd,
}

impl UnixEventSource {
	/// Creates a new UnixEventSource by obtaining a tty file descriptor.
	pub fn new() -> Result<Self> {
		let tty = TtyFd::write()?;

		Ok(Self { tty })
	}
}

impl EventSource for UnixEventSource {
	fn try_read(
		&mut self,
		_timeout: Option<Duration>,
	) -> Result<Option<Event>> {
		let mut buf = [0u8; 1];

		match read(self.tty.fd(), &mut buf) {
			Ok(0) => Ok(None),
			Ok(_) => Ok(Some(Event::Key(AsciiKeyCode::from_ascii(buf[0])))),
			Err(e) if is_would_block(&e) => Ok(None),
			Err(e) => Err(e.into()),
		}
	}
}

/// Returns true if the error’s underlying source is an io::Error with kind
/// WouldBlock.
fn is_would_block(err: &TermError) -> bool {
	if let Some(source) = err.source() {
		if let Some(io_err) = source.downcast_ref::<std::io::Error>() {
			return io_err.kind() == std::io::ErrorKind::WouldBlock;
		}
	}
	false
}

fn read(fd: RawFd, buf: &mut [u8]) -> Result<usize> {
	let result =
		unsafe { ffi::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };

	match result {
		n if n > 0 => Ok(n as usize),
		0 => Ok(0), // No data read
		_ => Err(TermError::IoError(io::Error::last_os_error())),
	}
}

fn close(fd: RawFd) -> Result<()> {
	match unsafe { ffi::close(fd) } {
		0 => Ok(()),
		_ => Err(TermError::IoError(io::Error::last_os_error())),
	}
}
