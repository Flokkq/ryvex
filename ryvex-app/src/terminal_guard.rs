use std::{
	io::stdin,
	marker::PhantomData,
};

#[cfg(unix)]
use std::os::unix::io::{
	AsRawFd,
	RawFd,
};

#[cfg(windows)]
use std::os::windows::io::{
	AsRawHandle,
	RawHandle,
};

#[cfg(windows)]
use ryvex_term::sys::windows::ConsoleHandle;

use crate::error::Result;
use ryvex_term::sys::target::termios::Termios;

pub struct TerminalGuard<'a> {
	#[cfg(unix)]
	fd: RawFd,

	#[cfg(windows)]
	handle: ConsoleHandle,

	orig_termios: Termios,
	_phantom:     PhantomData<&'a ()>,
}
impl<'a> TerminalGuard<'a> {
	#[cfg(unix)]
	pub fn spawn() -> Result<Self> {
		let stdin = stdin();
		let stdin_fd = stdin.as_raw_fd();

		let mut termios = Termios::from_fd(stdin_fd)?;
		let orig_termios = termios.raw(stdin_fd)?;

		Ok(TerminalGuard {
			fd: stdin_fd,
			orig_termios,
			_phantom: std::marker::PhantomData,
		})
	}

	#[cfg(unix)]
	pub fn restore(&self) -> Result<()> {
		Ok(Termios::restore_terminal(self.fd, self.orig_termios)?)
	}

	#[cfg(windows)]
	pub fn spawn() -> Result<Self> {
		let handle = unsafe { ConsoleHandle::new(stdin().as_raw_handle()) };

		let mut t = Termios::from_handle(handle)?;
		let orig = t.raw(handle)?;

		Ok(Self {
			handle,
			orig_termios: orig,
			_phantom: PhantomData,
		})
	}

	#[cfg(windows)]
	pub fn restore(&self) -> Result<()> {
		Ok(Termios::restore_terminal(self.handle, self.orig_termios)?)
	}
}

impl<'a> Drop for TerminalGuard<'a> {
	fn drop(&mut self) {
		let _ = self.restore();
	}
}

unsafe impl<'a> Sync for TerminalGuard<'a> {}
unsafe impl<'a> Send for TerminalGuard<'a> {}
