use std::{
	fs::OpenOptions,
	os::fd::{
		IntoRawFd,
		RawFd,
	},
};

use crate::{
	std::{
		error::IoError,
		Result,
	},
	target::unix::target::STDIN_FILENO,
	term::console::Handle,
};

use super::ffi;

pub struct TtyFdSettings {
	read:  bool,
	write: bool,
}

impl TtyFdSettings {
	pub fn read() -> Self {
		Self {
			read:  true,
			write: false,
		}
	}

	pub fn write() -> Self {
		Self {
			read:  false,
			write: true,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TtyFd {
	fd:            RawFd,
	close_on_drop: bool,
}

impl TtyFd {
	pub fn from_default_tty(read: bool, write: bool) -> Result<Self> {
		let (fd, close_on_drop) = if ffi::isatty(STDIN_FILENO) {
			(STDIN_FILENO, false)
		} else {
			let file = OpenOptions::new()
				.read(read)
				.write(write)
				.open("/dev/tty")
				.map_err(IoError::from)?;
			(file.into_raw_fd(), true)
		};

		Ok(Self { fd, close_on_drop })
	}
}

impl Handle<RawFd, TtyFdSettings> for TtyFd {
	fn acquire(mode: TtyFdSettings) -> Result<Self> {
		TtyFd::from_default_tty(mode.read, mode.write)
	}

	fn inner(&self) -> &RawFd {
		&self.fd
	}

	fn inner_mut(&mut self) -> &mut RawFd {
		&mut self.fd
	}
}

impl Drop for TtyFd {
	fn drop(&mut self) {
		if self.close_on_drop {
			let _ = ffi::close(self.fd);
		}
	}
}
