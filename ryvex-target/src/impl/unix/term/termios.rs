use std::io;
use std::os::fd::AsRawFd;
use std::os::fd::RawFd;

use crate::std::error::IoError;
use crate::std::Result;
use crate::target::unix::target::{
	self,
	os::TCSANOW,
};
use crate::term::console::{
	Console,
	Handle,
};

use super::fd::{
	TtyFd,
	TtyFdSettings,
};
use super::ffi;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Termios {
	inner: target::os::termios,
}

impl Termios {
	pub fn from_fd(fd: RawFd) -> Result<Self> {
		let termios = ffi::tcgetattr(fd).map_err(IoError::from)?;

		Ok(termios)
	}

	pub(crate) fn inner(&self) -> &target::os::termios {
		&self.inner
	}

	pub(crate) fn inner_mut(&mut self) -> &mut target::os::termios {
		&mut self.inner
	}

	/// Sets the terminal to `raw` mode and returns the original termios
	/// configuration
	pub fn enter_raw_mode(&mut self, fd: RawFd) -> Result<Self> {
		let orig_termios = *self;

		ffi::cfmakeraw(self);
		ffi::tcsetattr(fd, TCSANOW, self).map_err(IoError::from)?;

		Ok(orig_termios)
	}

	/// Resets the terminal to the original termios configuration
	pub fn restore_terminal(
		fd: RawFd,
		orig_termios: Termios,
	) -> io::Result<()> {
		ffi::tcsetattr(fd, TCSANOW, &orig_termios)
	}
}

impl Console<RawFd, TtyFdSettings> for Termios {
	type Handle = TtyFd;

	fn init() -> Result<(Self, Self::Handle)> {
		let fd = TtyFd::acquire(TtyFdSettings::read())?;
		let termios = Termios::from_fd(fd.inner().as_raw_fd())?;

		Ok((termios, fd))
	}

	fn raw(&mut self, fd: &Self::Handle) -> Result<Self> {
		self.enter_raw_mode(fd.inner().as_raw_fd())
	}

	fn restore(fd: &Self::Handle, orig: Self) -> Result<()> {
		Ok(Termios::restore_terminal(fd.inner().as_raw_fd(), orig)
			.map_err(IoError::from)?)
	}
}
