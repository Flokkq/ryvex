use std::ffi::c_int;

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
}
