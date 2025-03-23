use std::ffi::{
	c_int,
	c_void,
};

use super::target;

#[link(name = "c")]
extern "C" {
	pub fn tcgetattr(fd: c_int, termios_p: *mut target::os::termios) -> c_int;
	pub fn cfmakeraw(termios_p: *mut target::os::termios);
	pub fn tcsetattr(
		fd: c_int,
		optional_actions: c_int,
		termios_p: *const target::os::termios,
	) -> c_int;

	pub fn isatty(fd: c_int) -> c_int;
	pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
	pub fn close(fd: c_int) -> c_int;
}
