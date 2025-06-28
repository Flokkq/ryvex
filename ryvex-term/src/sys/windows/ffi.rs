use std::ffi::{
	c_int,
	c_long,
	c_ulong,
	c_void,
};

pub type BOOL = c_int;
pub type HANDLE = *mut c_void;

pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;

pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

#[link(name = "kernel32")]
unsafe extern "system" {
	pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
	pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
	pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

#[link(name = "user32")]
unsafe extern "system" {
	pub fn SetCursorPos(X: c_int, Y: c_int) -> BOOL;
}
