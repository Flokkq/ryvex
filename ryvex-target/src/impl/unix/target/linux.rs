#![allow(non_camel_case_types)]

use std::ffi::{
	c_int,
	c_uchar,
	c_uint,
};

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	c_line:      cc_t,
	pub c_cc:    [cc_t; NCCS],
	c_ispeed:    speed_t,
	c_ospeed:    speed_t,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct winsize {
	pub ws_row:    u16,
	pub ws_col:    u16,
	pub ws_xpixel: u16,
	pub ws_ypixel: u16,
}

pub const NCCS: usize = 32;

// used for `tcsetattr`
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

// used for `ioctl`
pub const TIOCGWINSZ: c_int = 0x5413;
