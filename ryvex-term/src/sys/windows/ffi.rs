#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::{
	c_int,
	c_long,
	c_ulong,
	c_ushort,
	c_void,
};

pub type BOOL = c_int;
pub type SHORT = i16;
pub type HANDLE = *mut c_void;

pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;
pub type WCHAR = c_ushort;

pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
pub const GENERIC_READ: DWORD = 0x8000_0000;
pub const GENERIC_WRITE: DWORD = 0x4000_0000;
pub const FILE_SHARE_READ: DWORD = 0x0000_0001;
pub const FILE_SHARE_WRITE: DWORD = 0x0000_0002;
pub const OPEN_EXISTING: DWORD = 3;

pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
pub const ENABLE_ECHO_INPUT: DWORD = 0x0004;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct COORD {
	pub X: SHORT,
	pub Y: SHORT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SMALL_RECT {
	pub Left:   SHORT,
	pub Top:    SHORT,
	pub Right:  SHORT,
	pub Bottom: SHORT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
	pub dwSize:              COORD,
	pub dwCursorPosition:    COORD,
	pub wAttributes:         WORD,
	pub srWindow:            SMALL_RECT,
	pub dwMaximumWindowSize: COORD,
}

pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CONSOLE_CURSOR_INFO {
	pub dwSize:   DWORD,
	pub bVisible: BOOL,
}

pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;

#[repr(C)]
#[derive(Clone, Copy)]
pub union CHAR_INFO_Char {
	pub UnicodeChar: u16,
	pub AsciiChar:   u8,
}

impl std::fmt::Debug for CHAR_INFO_Char {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe { write!(f, "CHAR_INFO_Char({})", self.UnicodeChar) }
	}
}

impl PartialEq for CHAR_INFO_Char {
	fn eq(&self, other: &Self) -> bool {
		unsafe { self.UnicodeChar == other.UnicodeChar }
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CHAR_INFO {
	pub Char:       CHAR_INFO_Char,
	pub Attributes: WORD,
}

#[link(name = "kernel32")]
unsafe extern "system" {
	pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
	pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
	pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
	pub fn GetConsoleScreenBufferInfo(
		hConsoleOutput: HANDLE,
		lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
	) -> BOOL;
	pub fn ScrollConsoleScreenBufferW(
		hConsoleOutput: HANDLE,
		lpScrollRectangle: *const SMALL_RECT,
		lpClipRectangle: *const SMALL_RECT,
		dwDestinationOrigin: COORD,
		lpFill: *const CHAR_INFO,
	) -> BOOL;
	pub fn FillConsoleOutputCharacterW(
		hConsoleOutput: HANDLE,
		cCharacter: WCHAR,
		nLength: DWORD,
		dwWriteCoord: COORD,
		lpNumberOfCharsWritten: LPDWORD,
	) -> BOOL;
	pub fn FillConsoleOutputAttribute(
		hConsoleOutput: HANDLE,
		wAttribute: WORD,
		nLength: DWORD,
		dwWriteCoord: COORD,
		lpNumberOfAttrsWritten: LPDWORD,
	) -> BOOL;
	pub fn WriteConsoleW(
		hConsoleOutput: HANDLE,
		lpBuffer: *const WCHAR,
		nNumberOfCharsToWrite: DWORD,
		lpNumberOfCharsWritten: LPDWORD,
		lpReserved: *mut c_void,
	) -> BOOL;
	pub fn CreateFileA(
		lpFileName: *const i8,
		dwDesiredAccess: DWORD,
		dwShareMode: DWORD,
		lpSecurityAttributes: *mut c_void,
		dwCreationDisposition: DWORD,
		dwFlagsAndAttributes: DWORD,
		hTemplateFile: HANDLE,
	) -> HANDLE;
	pub fn CloseHandle(hObject: HANDLE) -> BOOL;
}

#[link(name = "user32")]
unsafe extern "system" {
	pub fn SetConsoleCursorPosition(
		hConsoleOutput: HANDLE,
		dwCursorPosition: COORD,
	) -> BOOL;
	pub fn SetConsoleCursorInfo(
		hConsoleOutput: HANDLE,
		lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO,
	) -> BOOL;
	pub fn GetConsoleCursorInfo(
		hConsoleOutput: HANDLE,
		lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO,
	) -> BOOL;
	pub fn SetConsoleScreenBufferSize(
		hConsoleOutput: HANDLE,
		dwSize: COORD,
	) -> BOOL;
	pub fn SetConsoleWindowInfo(
		hConsoleOutput: HANDLE,
		bAbsolute: BOOL,
		lpConsoleWindow: *const SMALL_RECT,
	) -> BOOL;
	pub fn GetLargestConsoleWindowSize(hConsoleOutput: HANDLE) -> COORD;
}

#[link(name = "msvcrt")]
extern "C" {
	pub fn _kbhit() -> c_int;
	pub fn _getch() -> c_int;
}
