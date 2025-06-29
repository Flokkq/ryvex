use std::{
	ffi::{
		c_int,
		c_ulong,
	},
	mem::MaybeUninit,
};

use error::TermError;
use ryvex_ui::graphics::Rect;
use sys::unix::{
	fd::TtyFd,
	target::os::TIOCGWINSZ,
};

use crate::{
	error::Result,
	sys::unix::target::os::winsize,
};

pub mod command;
pub mod cursor;
pub mod display;
pub mod error;
pub mod event;
pub mod key;
pub mod macros;
pub mod sys;
pub mod termios;

pub fn get_terminal_size(fd: &TtyFd) -> Result<Rect> {
	// unwrap is safe because we know that `TIOCGWINSZ` always fits into a
	// `c_ulong`
	let winsize = ioctl(fd, TIOCGWINSZ.try_into().unwrap())?;

	Ok(Rect {
		width:  winsize.ws_col,
		height: winsize.ws_row,
		x:      winsize.ws_xpixel,
		y:      winsize.ws_ypixel,
	})
}

fn ioctl(fd: &TtyFd, request: c_ulong) -> Result<winsize> {
	let mut winsize = MaybeUninit::<winsize>::uninit();

	io_result(unsafe {
		sys::unix::ffi::ioctl(fd.fd(), request, winsize.as_mut_ptr())
	})?;

	Ok(unsafe { winsize.assume_init() })
}

fn io_result(result: c_int) -> Result<()> {
	if result == -1 {
		Err(TermError::TerminalSizeError(std::io::Error::last_os_error()))
	} else {
		Ok(())
	}
}
