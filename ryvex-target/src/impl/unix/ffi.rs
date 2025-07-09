use std::{
	ffi::{
		c_int,
		c_ulong,
		c_void,
	},
	io,
	mem::MaybeUninit,
	os::fd::{
		AsRawFd,
		RawFd,
	},
};

use crate::term::console::Handle;

use super::{
	fd::TtyFd,
	target::{
		self,
		os::winsize,
	},
	termios::Termios,
};

#[link(name = "c")]
extern "C" {
	#[link_name = "tcgetattr"]
	fn c_tcgetattr(fd: c_int, termios_p: *mut target::os::termios) -> c_int;
	#[link_name = "cfmakeraw"]
	fn c_cfmakeraw(termios_p: *mut target::os::termios);
	#[link_name = "tcsetattr"]
	fn c_tcsetattr(
		fd: c_int,
		optional_actions: c_int,
		termios_p: *const target::os::termios,
	) -> c_int;

	#[link_name = "isatty"]
	fn c_isatty(fd: c_int) -> c_int;
	#[link_name = "read"]
	fn c_read(fd: c_int, buf: *mut c_void, count: usize) -> target::ssize_t;
	#[link_name = "close"]
	fn c_close(fd: c_int) -> c_int;

	#[link_name = "ioctl"]
	fn c_ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

pub fn tcsetattr(
	fd: RawFd,
	action: c_int,
	termios: &Termios,
) -> io::Result<()> {
	io_result(unsafe { c_tcsetattr(fd, action, termios.inner()) })
}

pub fn tcgetattr(fd: RawFd) -> io::Result<Termios> {
	let mut termios = MaybeUninit::<Termios>::uninit();
	let termios_ptr = termios.as_mut_ptr();

	io_result(unsafe { c_tcgetattr(fd, &mut *(*termios_ptr).inner_mut()) })?;

	Ok(unsafe { termios.assume_init() })
}

pub fn cfmakeraw(termios: &mut Termios) {
	unsafe { c_cfmakeraw(termios.inner_mut()) };
}

pub fn isatty(fd: RawFd) -> bool {
	unsafe { c_isatty(fd) == 1 }
}

pub fn close(fd: RawFd) -> io::Result<()> {
	io_result(unsafe { c_close(fd) })
}

pub fn read(fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
	let result = unsafe { c_read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };

	match result {
		n if n > 0 => Ok(n as usize),
		0 => Ok(0), // No data read
		_ => Err(io::Error::last_os_error()),
	}
}

pub fn ioctl(fd: &TtyFd, request: c_ulong) -> io::Result<winsize> {
	let mut winsize = MaybeUninit::<winsize>::uninit();

	io_result(unsafe {
		c_ioctl(fd.inner().as_raw_fd(), request, winsize.as_mut_ptr())
	})?;

	Ok(unsafe { winsize.assume_init() })
}

#[inline]
fn io_result(result: c_int) -> io::Result<()> {
	match result {
		0 => Ok(()),
		_ => Err(io::Error::last_os_error()),
	}
}
