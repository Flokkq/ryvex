use std::ffi::{
	c_int,
	c_long,
	c_ulong,
	c_void,
    c_ushort,
};

pub type BOOL = c_int;
pub type SHORT = i16;
pub type HANDLE = *mut c_void;

pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;

pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

#[repr(C)]
pub struct COORD {
	pub X: SHORT,
	pub Y: SHORT,
}

#[repr(C)]
pub struct SMALL_RECT {
    pub Left: SHORT,
    pub Top: SHORT,
    pub Right: SHORT,
    pub Bottom: SHORT,
}

#[repr(C)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: WORD,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}

pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

#[link(name = "kernel32")]
unsafe extern "system" {
	pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
	pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
	pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
}

#[link(name = "user32")]
unsafe extern "system" {
	pub fn SetConsoleCursorPosition(
		hConsoleOutput: HANDLE,
		dwCursorPosition: COORD,
	) -> BOOL;
}
