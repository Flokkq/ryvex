use std::{
	ffi::c_int,
	io,
	mem::MaybeUninit,
	os::fd::RawFd,
};

use self::unix::target::os::TCSANOW;
use crate::error::Result;

mod unix;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Termios {
	inner: unix::target::os::termios,
}

impl Termios {
	pub fn from_fd(fd: RawFd) -> Result<Self> {
		let mut termios = MaybeUninit::<Termios>::uninit();
		let termios_ptr = termios.as_mut_ptr();

		io_result(unsafe {
			unix::ffi::tcgetattr(fd, &mut (*termios_ptr).inner)
		})?;

		Ok(unsafe { termios.assume_init() })
	}

	fn inner(&self) -> &unix::target::os::termios {
		&self.inner
	}

	fn inner_mut(&mut self) -> &mut unix::target::os::termios {
		&mut self.inner
	}

	/// Sets the terminal to `raw` mode and returns the original termios
	/// configuration
	pub fn raw(&mut self, fd: RawFd) -> Result<Self> {
		let orig_termios = self.clone();

		unsafe {
			unix::ffi::cfmakeraw(self.inner_mut());
		};
		tcsetattr(fd, TCSANOW, &self)?;

		Ok(orig_termios)
	}

	/// Resets the terminal to the original termios configuration
	pub fn restore_terminal(fd: RawFd, orig_termios: Termios) -> Result<()> {
		tcsetattr(fd, TCSANOW, &orig_termios)
	}
}

fn tcsetattr(fd: RawFd, action: c_int, termios: &Termios) -> Result<()> {
	io_result(unsafe { unix::ffi::tcsetattr(fd, action, termios.inner()) })
}

#[inline]
fn io_result(result: c_int) -> Result<()> {
	match result {
		0 => Ok(()),
		_ => Err(io::Error::last_os_error().into()),
	}
}
