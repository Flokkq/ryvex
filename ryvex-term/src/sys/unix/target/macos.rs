#![allow(non_camel_case_types)]

use std::ffi::{
	c_int,
	c_uchar,
	c_ulong,
	c_ushort,
};

pub type tcflag_t = c_ulong;
pub type cc_t = c_uchar;
pub type speed_t = c_ulong;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_cc:    [cc_t; NCCS],
	c_ispeed:    speed_t,
	c_ospeed:    speed_t,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct winsize {
	pub ws_row:    c_ushort,
	pub ws_col:    c_ushort,
	pub ws_xpixel: c_ushort,
	pub ws_ypixel: c_ushort,
}

pub const NCCS: usize = 20;

// `tcsetattr`
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const TCSASOFT: c_int = 0x10;

// used for `ioctl`
pub const TIOCGWINSZ: c_int = 0x40087468;
