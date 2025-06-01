use std::{
	io::stdin,
	os::fd::AsRawFd,
};

use crate::error::Result;
use ryvex_term::termios::Termios;

pub struct TerminalGuard<'a> {
	fd:           i32,
	orig_termios: Termios,
	_phantom:     std::marker::PhantomData<&'a ()>,
}

impl<'a> TerminalGuard<'a> {
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

	pub fn restore(&self) -> Result<()> {
		Ok(Termios::restore_terminal(self.fd, self.orig_termios)?)
	}
}

impl<'a> Drop for TerminalGuard<'a> {
	fn drop(&mut self) {
		let _ = self.restore();
	}
}
