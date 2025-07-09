#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::{
	ffi::{
		c_int,
		c_ulong,
		c_ushort,
		c_void,
	},
	io,
	mem::MaybeUninit,
	os::windows::io::RawHandle,
	ptr,
};

pub type BOOL = c_int;
pub type SHORT = i16;
pub type HANDLE = RawHandle;

pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;
pub type WCHAR = c_ushort;

pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
pub const STD_INPUT_HANDLE: DWORD = -10i32 as DWORD;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
	fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
	fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
	fn GetConsoleScreenBufferInfo(
		hConsoleOutput: HANDLE,
		lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
	) -> BOOL;
	fn ScrollConsoleScreenBufferW(
		hConsoleOutput: HANDLE,
		lpScrollRectangle: *const SMALL_RECT,
		lpClipRectangle: *const SMALL_RECT,
		dwDestinationOrigin: COORD,
		lpFill: *const CHAR_INFO,
	) -> BOOL;
	fn FillConsoleOutputCharacterW(
		hConsoleOutput: HANDLE,
		cCharacter: WCHAR,
		nLength: DWORD,
		dwWriteCoord: COORD,
		lpNumberOfCharsWritten: LPDWORD,
	) -> BOOL;
	fn FillConsoleOutputAttribute(
		hConsoleOutput: HANDLE,
		wAttribute: WORD,
		nLength: DWORD,
		dwWriteCoord: COORD,
		lpNumberOfAttrsWritten: LPDWORD,
	) -> BOOL;
	fn WriteConsoleW(
		hConsoleOutput: HANDLE,
		lpBuffer: *const WCHAR,
		nNumberOfCharsToWrite: DWORD,
		lpNumberOfCharsWritten: LPDWORD,
		lpReserved: *mut c_void,
	) -> BOOL;
	fn CreateFileA(
		lpFileName: *const i8,
		dwDesiredAccess: DWORD,
		dwShareMode: DWORD,
		lpSecurityAttributes: *mut c_void,
		dwCreationDisposition: DWORD,
		dwFlagsAndAttributes: DWORD,
		hTemplateFile: HANDLE,
	) -> HANDLE;
	fn CloseHandle(hObject: HANDLE) -> BOOL;
}

#[link(name = "user32")]
unsafe extern "system" {
	fn SetConsoleCursorPosition(
		hConsoleOutput: HANDLE,
		dwCursorPosition: COORD,
	) -> BOOL;
	fn SetConsoleCursorInfo(
		hConsoleOutput: HANDLE,
		lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO,
	) -> BOOL;
	fn GetConsoleCursorInfo(
		hConsoleOutput: HANDLE,
		lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO,
	) -> BOOL;
	fn SetConsoleScreenBufferSize(
		hConsoleOutput: HANDLE,
		dwSize: COORD,
	) -> BOOL;
	fn SetConsoleWindowInfo(
		hConsoleOutput: HANDLE,
		bAbsolute: BOOL,
		lpConsoleWindow: *const SMALL_RECT,
	) -> BOOL;
	fn GetLargestConsoleWindowSize(hConsoleOutput: HANDLE) -> COORD;
}

#[link(name = "msvcrt")]
extern "C" {
	pub fn _kbhit() -> c_int;
	pub fn _getch() -> c_int;
}

pub fn get_current_out_handle() -> io::Result<HANDLE> {
	let h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

	ok_or_last_error(h, h.is_null() || h == INVALID_HANDLE_VALUE)
}

pub fn get_current_in_handle() -> io::Result<HANDLE> {
	let h = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

	ok_or_last_error(h, h.is_null() || h == INVALID_HANDLE_VALUE)
}

pub fn get_console_mode_from_handle(handle: HANDLE) -> io::Result<DWORD> {
	let mut mode: DWORD = 0;

	ok_if_nonzero(mode, unsafe { GetConsoleMode(handle, ptr_out(&mut mode)) })
}

pub fn set_console_mode(handle: HANDLE, mode: DWORD) -> io::Result<()> {
	io_result(unsafe { SetConsoleMode(handle, mode) })
}

pub fn get_screen_buffer_info(
	handle: HANDLE,
) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO> {
	let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();
	let ret = unsafe { GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) };

	ok_if_nonzero(unsafe { info.assume_init() }, ret)
}

pub fn scroll_screen_buffer(
	handle: HANDLE,
	scroll_rect: Option<&SMALL_RECT>,
	clip_rect: Option<&SMALL_RECT>,
	dest: COORD,
	fill: CHAR_INFO,
) -> io::Result<()> {
	let ret = unsafe {
		ScrollConsoleScreenBufferW(
			handle,
			ptr_opt(scroll_rect),
			ptr_opt(clip_rect),
			dest,
			&fill,
		)
	};

	io_result(ret)
}

pub fn fill_console_output_character(
	handle: HANDLE,
	ch: WCHAR,
	len: DWORD,
	coord: COORD,
) -> io::Result<()> {
	let mut written: DWORD = 0;
	let ret = unsafe {
		FillConsoleOutputCharacterW(
			handle,
			ch,
			len,
			coord,
			ptr_out(&mut written),
		)
	};

	io_result(ret)
}

pub fn fill_console_output_attribute(
	handle: HANDLE,
	attr: WORD,
	len: DWORD,
	coord: COORD,
) -> io::Result<()> {
	let mut written: DWORD = 0;
	let ret = unsafe {
		FillConsoleOutputAttribute(
			handle,
			attr,
			len,
			coord,
			ptr_out(&mut written),
		)
	};

	io_result(ret)
}

pub fn write_console(handle: HANDLE, text: &str) -> io::Result<()> {
	let utf16: Vec<WCHAR> = text.encode_utf16().collect();
	let mut written: DWORD = 0;
	let ret = unsafe {
		WriteConsoleW(
			handle,
			utf16.as_ptr(),
			utf16.len() as DWORD,
			ptr_out(&mut written),
			ptr::null_mut(),
		)
	};

	io_result(ret)
}

pub fn set_cursor_position(handle: HANDLE, pos: COORD) -> io::Result<()> {
	unsafe { io_result(SetConsoleCursorPosition(handle, pos)) }
}

pub fn get_cursor_info(handle: HANDLE) -> io::Result<CONSOLE_CURSOR_INFO> {
	let mut info = MaybeUninit::<CONSOLE_CURSOR_INFO>::uninit();
	let ret = unsafe { GetConsoleCursorInfo(handle, info.as_mut_ptr()) };
	ok_if_nonzero(unsafe { info.assume_init() }, ret)
}

pub fn set_cursor_info(
	handle: HANDLE,
	info: &CONSOLE_CURSOR_INFO,
) -> io::Result<()> {
	unsafe { io_result(SetConsoleCursorInfo(handle, info)) }
}

pub fn set_console_screen_buffer_size(
	handle: HANDLE,
	size: COORD,
) -> io::Result<()> {
	unsafe { io_result(SetConsoleScreenBufferSize(handle, size)) }
}

pub fn set_console_window_info(
	handle: HANDLE,
	absolute: bool,
	window: &SMALL_RECT,
) -> io::Result<()> {
	let flag: BOOL = if absolute { 1 } else { 0 };
	unsafe { io_result(SetConsoleWindowInfo(handle, flag, window)) }
}

pub fn get_largest_console_window_size(handle: HANDLE) -> io::Result<COORD> {
	let coord = unsafe { GetLargestConsoleWindowSize(handle) };
	ok_or_last_error(coord, coord.X == 0 && coord.Y == 0)
}

pub fn create_file_a(
	file_name: *const i8,
	desired_access: u32,
	share_mode: u32,
	security_attributes: *mut c_void,
	creation_disposition: u32,
	flags_and_attributes: u32,
	template_file: HANDLE,
) -> io::Result<HANDLE> {
	let h = unsafe {
		CreateFileA(
			file_name,
			desired_access,
			share_mode,
			security_attributes,
			creation_disposition,
			flags_and_attributes,
			template_file,
		)
	};
	ok_or_last_error(h, h.is_null() || h == INVALID_HANDLE_VALUE)
}

pub fn close_handle(handle: HANDLE) -> io::Result<()> {
	unsafe { io_result(CloseHandle(handle)) }
}

pub fn kbhit() -> bool {
	unsafe { _kbhit() != 0 }
}

pub fn getch() -> i32 {
	unsafe { _getch() }
}

#[inline]
fn ptr_opt<T>(opt: Option<&T>) -> *const T {
	opt.map_or(ptr::null(), |r| r as *const _)
}

#[inline]
fn ptr_out<T>(out: &mut T) -> *mut T {
	out as *mut _
}

#[inline]
fn ok_or_last_error<T>(value: T, is_error: bool) -> io::Result<T> {
	if is_error {
		Err(io::Error::last_os_error())
	} else {
		Ok(value)
	}
}

#[inline]
fn ok_if_nonzero<T>(value: T, ret: c_int) -> io::Result<T> {
	if ret != 0 {
		Ok(value)
	} else {
		Err(io::Error::last_os_error())
	}
}

#[inline]
fn io_result(result: c_int) -> io::Result<()> {
	match result {
		0 => Err(io::Error::last_os_error()),
		_ => Ok(()),
	}
}
