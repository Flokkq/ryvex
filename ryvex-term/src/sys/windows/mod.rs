pub mod ffi;

use ffi::{
	GetConsoleMode,
	GetStdHandle,
	SetConsoleCursorPosition,
	SetConsoleMode,
	COORD,
	DWORD,
	ENABLE_VIRTUAL_TERMINAL_PROCESSING,
	HANDLE,
	INVALID_HANDLE_VALUE,
	LPDWORD,
	STD_OUTPUT_HANDLE,
};
use std::io;
use std::sync::OnceLock;

static SUPPORTS_ANSI: OnceLock<bool> = OnceLock::new();

pub fn supports_ansi() -> bool {
	*SUPPORTS_ANSI.get_or_init(|| enable_vt_processing().is_ok())
}

pub fn enable_vt_processing() -> io::Result<()> {
	let mask = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

	let handle = unsafe { get_current_out_handle()? };
	let console_mode = unsafe { get_console_mode_from_handle(handle)? };

	if console_mode & mask == 0 {
		let mode = console_mode | mask;
		unsafe { set_console_mode(handle, mode)? };
	}

	Ok(())
}

pub fn move_cursor(x: i16, y: i16) -> io::Result<()> {
	if x < 0 {
		return Err(io::Error::new(
			io::ErrorKind::Other,
			format!("cursor position out of range - X: {x}"),
		));
	}

	if y < 0 {
		return Err(io::Error::new(
			io::ErrorKind::Other,
			format!("cursor position out of range - Y: {y}"),
		));
	}

	let point = COORD { X: x, Y: y };
	let handle = unsafe { (get_current_out_handle())? };

	unsafe { set_cursor_pos(handle, point) }
}

unsafe fn get_current_out_handle() -> io::Result<HANDLE> {
	let handle = GetStdHandle(STD_OUTPUT_HANDLE);
	if handle == INVALID_HANDLE_VALUE {
		Err(io::Error::last_os_error())
	} else {
		Ok(handle)
	}
}

unsafe fn get_console_mode_from_handle(handle: HANDLE) -> io::Result<DWORD> {
	let mut mode: DWORD = 0;
	if GetConsoleMode(handle, &mut mode as LPDWORD) == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(mode)
	}
}

unsafe fn set_console_mode(handle: HANDLE, mode: DWORD) -> io::Result<()> {
	if SetConsoleMode(handle, mode) == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(())
	}
}

unsafe fn set_cursor_pos(handle: HANDLE, pos: COORD) -> io::Result<()> {
	if SetConsoleCursorPosition(handle, pos) == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(())
	}
}
