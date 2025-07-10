use std::{
	error::Error,
	io,
	os::fd::AsRawFd,
	time::Duration,
};

use super::{
	fd::TtyFd,
	ffi,
};
use crate::{
	key::AsciiKeyCode,
	term::{
		console::Handle,
		event::{
			Event,
			EventSource,
		},
	},
};

/// Unix event source implementation.
pub struct UnixEventSource {
	tty: TtyFd,
}

impl UnixEventSource {
	/// Creates a new UnixEventSource by obtaining a tty file descriptor.
	pub fn new() -> io::Result<Self> {
		let tty = TtyFd::from_default_tty(false, true)?;

		Ok(Self { tty })
	}
}

impl EventSource for UnixEventSource {
	fn try_read(
		&mut self,
		_timeout: Option<Duration>,
	) -> io::Result<Option<Event>> {
		let mut buf = [0u8; 1];

		match ffi::read(self.tty.inner().as_raw_fd(), &mut buf) {
			Ok(0) => Ok(None),
			Ok(_) => Ok(Some(Event::Key(AsciiKeyCode::from_ascii(buf[0])))),
			Err(e) if is_would_block(&e) => Ok(None),
			Err(e) => Err(e),
		}
	}
}

/// Returns true if the error’s underlying source is an io::Error with kind
/// WouldBlock.
fn is_would_block(err: &dyn Error) -> bool {
	if let Some(source) = err.source() {
		if let Some(io_err) = source.downcast_ref::<std::io::Error>() {
			return io_err.kind() == std::io::ErrorKind::WouldBlock;
		}
	}
	false
}
