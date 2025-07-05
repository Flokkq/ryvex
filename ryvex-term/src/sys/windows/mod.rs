#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod fd;
pub mod ffi;
pub mod termios;

use crate::error::Result;
use crate::sys::windows::fd::TtyFd;
use crate::terminal::ClearType;
use ffi::{
	CHAR_INFO_Char,
	CloseHandle,
	CreateFileA,
	FillConsoleOutputAttribute,
	FillConsoleOutputCharacterW,
	GetConsoleCursorInfo,
	GetConsoleMode,
	GetConsoleScreenBufferInfo,
	GetLargestConsoleWindowSize,
	GetStdHandle,
	ScrollConsoleScreenBufferW,
	SetConsoleCursorInfo,
	SetConsoleCursorPosition,
	SetConsoleMode,
	SetConsoleScreenBufferSize,
	SetConsoleWindowInfo,
	WriteConsoleW,
	CHAR_INFO,
	CONSOLE_CURSOR_INFO,
	CONSOLE_SCREEN_BUFFER_INFO,
	COORD,
	DWORD,
	ENABLE_VIRTUAL_TERMINAL_PROCESSING,
	FILE_SHARE_READ,
	FILE_SHARE_WRITE,
	HANDLE,
	INVALID_HANDLE_VALUE,
	LPDWORD,
	OPEN_EXISTING,
	SMALL_RECT,
	STD_OUTPUT_HANDLE,
	WORD,
};
use ryvex_ui::graphics::Rect;
use std::ffi::c_void;
use std::ffi::CStr;
use std::sync::{
	atomic::{
		AtomicU64,
		Ordering,
	},
	OnceLock,
};
use std::{
	io,
	ptr,
};

static SUPPORTS_ANSI: OnceLock<bool> = OnceLock::new();

static SAVED_CURSOR_POS: AtomicU64 = AtomicU64::new(u64::MAX);

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

pub fn move_to(x: i16, y: i16) -> io::Result<()> {
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

pub fn move_up(count: u16) -> io::Result<()> {
	let (col, row) = current_cursor_pos()?;
	move_to(col, row - count as i16)
}

pub fn move_down(count: u16) -> io::Result<()> {
	let (col, row) = current_cursor_pos()?;
	move_to(col, row + count as i16)
}

pub fn move_left(count: u16) -> io::Result<()> {
	let (col, row) = current_cursor_pos()?;
	move_to(col - count as i16, row)
}

pub fn move_right(count: u16) -> io::Result<()> {
	let (col, row) = current_cursor_pos()?;
	move_to(col + count as i16, row)
}

pub fn move_to_next_line(count: u16) -> io::Result<()> {
	let (_, row) = current_cursor_pos()?;
	move_to(0, row + count as i16)
}

pub fn move_to_previous_line(count: u16) -> io::Result<()> {
	let (_, row) = current_cursor_pos()?;
	move_to(0, row - count as i16)
}

pub fn move_to_row(new_row: u16) -> io::Result<()> {
	let (col, _) = current_cursor_pos()?;
	move_to(col, new_row as i16)
}

pub fn move_to_column(new_col: u16) -> io::Result<()> {
	let (_, row) = current_cursor_pos()?;
	move_to(new_col as i16, row)
}

pub fn save_position() -> io::Result<()> {
	let (x, y) = current_cursor_pos()?;

	// `x` is stored in the first 16 bits
	let upper = u32::from(x as u16) << 16;

	// `y` is stored in the last 16 bits
	let lower = u32::from(y as u16);

	// combine into one singular u32
	SAVED_CURSOR_POS.store(u64::from(upper | lower), Ordering::Relaxed);

	Ok(())
}

pub fn restore_position() -> io::Result<()> {
	if let Ok(bits) = u32::try_from(SAVED_CURSOR_POS.load(Ordering::Relaxed)) {
		let x = ((bits >> 16) & 0xFFFF) as u16 as i16;
		let y = (bits & 0xFFFF) as u16 as i16;
		move_to(x, y)?;
	}

	Ok(())
}

pub fn current_cursor_pos() -> io::Result<(i16, i16)> {
	let handle = unsafe { (get_current_out_handle())? };
	let info = unsafe { get_screen_buffer_info(handle)? };

	Ok((info.dwCursorPosition.X, info.dwCursorPosition.Y))
}

pub fn show_cursor(show: bool) -> io::Result<()> {
	let handle = unsafe { (get_current_out_handle())? };

	let mut info = unsafe { get_console_cursor_info(handle)? };
	info.bVisible = if show { 1 } else { 0 };

	unsafe { set_console_cursor_info(handle, info) }
}

pub fn scroll_up(count: u16) -> io::Result<()> {
	scroll_to(count as i16)
}

pub fn scroll_down(count: u16) -> io::Result<()> {
	scroll_to(-(count as i16))
}

pub fn clear(clear_type: ClearType) -> io::Result<()> {
	let handle = unsafe { get_current_out_handle()? };
	let csbi = unsafe { get_screen_buffer_info(handle)? };
	let sr = csbi.srWindow;
	let attr = csbi.wAttributes;
	let width = (sr.Right - sr.Left + 1) as u32;
	let height = (sr.Bottom - sr.Top + 1) as u32;
	let cursor = csbi.dwCursorPosition;

	match clear_type {
		ClearType::All | ClearType::Purge => {
			let origin = COORD {
				X: sr.Left,
				Y: sr.Top,
			};

			unsafe {
				fill_region(handle, origin, width * height, attr, Some(origin))
			}
		}
		ClearType::FromCursorDown => {
			let consumed = (cursor.Y as u32 - sr.Top as u32) * width +
				(cursor.X as u32 - sr.Left as u32);
			let remaining = width * height - consumed;

			unsafe { fill_region(handle, cursor, remaining, attr, None) }
		}
		ClearType::FromCursorUp => {
			let length = (cursor.Y as u32 - sr.Top as u32 + 1) * width;
			let start = COORD {
				X: sr.Left,
				Y: sr.Top,
			};

			unsafe { fill_region(handle, start, length, attr, None) }
		}
		ClearType::CurrentLine => {
			let start = COORD {
				X: sr.Left,
				Y: cursor.Y,
			};

			unsafe { fill_region(handle, start, width, attr, None) }
		}
		ClearType::UntilNewLine => {
			let remaining = width - (cursor.X as u32 - sr.Left as u32);

			unsafe { fill_region(handle, cursor, remaining, attr, None) }
		}
	}
}

fn scroll_to(count: i16) -> io::Result<()> {
	let handle = unsafe { get_current_out_handle()? };
	let csbi = unsafe { get_screen_buffer_info(handle)? };
	let rect = csbi.srWindow;

	let dest = COORD {
		X: rect.Left,
		Y: rect.Top as i16 + count,
	};

	let ch = CHAR_INFO_Char {
		UnicodeChar: b' ' as u16,
	};

	let fill = CHAR_INFO {
		Char:       ch,
		Attributes: csbi.wAttributes,
	};

	unsafe {
		scroll_console_screen_buffer_w(handle, Some(&rect), None, dest, fill)
	}
}

pub fn set_size(width: u16, height: u16) -> io::Result<()> {
	if width < 1 || height < 1 {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"width and height must be at least 1",
		));
	}

	let handle = unsafe { get_current_out_handle()? };
	let csbi = unsafe { get_screen_buffer_info(handle)? };
	let orig_buf = csbi.dwSize;
	let win = csbi.srWindow;
	let w = width as i16;
	let h = height as i16;
	let mut new_buf = orig_buf;
	let mut resized_buffer = false;

	if orig_buf.X < win.Left + w {
		new_buf.X = win.Left + w;
		resized_buffer = true;
	}
	if orig_buf.Y < win.Top + h {
		new_buf.Y = win.Top + h;
		resized_buffer = true;
	}
	if resized_buffer {
		unsafe {
			set_console_screen_buffer_size(handle, new_buf)?;
		}
	}

	let mut new_win = win;
	new_win.Right = win.Left + w - 1;
	new_win.Bottom = win.Top + h - 1;
	unsafe {
		set_console_window_info(handle, true, &new_win)?;
	}

	if resized_buffer {
		unsafe {
			set_console_screen_buffer_size(handle, orig_buf)?;
		}
	}

	let max_win = unsafe { get_largest_console_window_size(handle)? };
	if w > max_win.X {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			format!("terminal width {} too large (max {})", width, max_win.X),
		));
	}
	if h > max_win.Y {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			format!("terminal height {} too large (max {})", height, max_win.Y),
		));
	}

	Ok(())
}

pub fn write(text: &str) -> io::Result<()> {
	let handle = unsafe { get_current_out_handle()? };

	unsafe { write_console(handle, text) }
}

pub fn open_device(name: *const i8, access: u32) -> io::Result<HANDLE> {
	let handle = unsafe {
		create_file_a(
			name,
			access,
			FILE_SHARE_READ | FILE_SHARE_WRITE,
			ptr::null_mut(),
			OPEN_EXISTING,
			0,
			ptr::null_mut(),
		)?
	};

	Ok(handle)
}

pub fn get_terminal_size(fd: &TtyFd) -> Result<Rect> {
	let handle = fd.handle();

	let info = unsafe { get_screen_buffer_info(handle)? };

	let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
	let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;

	Ok(Rect {
		x: 0,
		y: 0,
		width,
		height,
	})
}

pub(crate) unsafe fn get_current_out_handle() -> io::Result<HANDLE> {
	let handle = GetStdHandle(STD_OUTPUT_HANDLE);
	if handle == INVALID_HANDLE_VALUE || handle.is_null() {
		Err(io::Error::last_os_error())
	} else {
		Ok(handle)
	}
}

pub(crate) unsafe fn get_console_mode_from_handle(
	handle: HANDLE,
) -> io::Result<DWORD> {
	let mut mode: DWORD = 0;
	if GetConsoleMode(handle, &mut mode as LPDWORD) == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(mode)
	}
}

pub(crate) unsafe fn set_console_mode(
	handle: HANDLE,
	mode: DWORD,
) -> io::Result<()> {
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

unsafe fn get_screen_buffer_info(
	handle: HANDLE,
) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO> {
	let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

	if GetConsoleScreenBufferInfo(handle, &mut info) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(info)
}

unsafe fn get_console_cursor_info(
	handle: HANDLE,
) -> io::Result<CONSOLE_CURSOR_INFO> {
	let mut info: CONSOLE_CURSOR_INFO = std::mem::zeroed();

	if GetConsoleCursorInfo(handle, &mut info) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(info)
}

unsafe fn set_console_cursor_info(
	handle: HANDLE,
	info: CONSOLE_CURSOR_INFO,
) -> io::Result<()> {
	if SetConsoleCursorInfo(handle, &info as *const _) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn scroll_console_screen_buffer_w(
	handle: HANDLE,
	scroll_rect: Option<&SMALL_RECT>,
	clip_rect: Option<&SMALL_RECT>,
	dest: COORD,
	fill: CHAR_INFO,
) -> io::Result<()> {
	let scroll_ptr =
		scroll_rect.map_or(ptr::null(), |r| r as *const SMALL_RECT);
	let clip_ptr = clip_rect.map_or(ptr::null(), |r| r as *const SMALL_RECT);

	let success = ScrollConsoleScreenBufferW(
		handle,
		scroll_ptr,
		clip_ptr,
		dest,
		&fill as *const _,
	);
	if success == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(())
	}
}

unsafe fn fill_console_output_character(
	handle: HANDLE,
	fill_char: u16,
	length: u32,
	coord: COORD,
) -> io::Result<()> {
	let mut written: DWORD = 0;
	let result = FillConsoleOutputCharacterW(
		handle,
		fill_char,
		length,
		coord,
		&mut written,
	);

	if result == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn fill_console_output_attribute(
	handle: HANDLE,
	attr: WORD,
	length: u32,
	coord: COORD,
) -> io::Result<()> {
	let mut written: DWORD = 0;
	let result =
		FillConsoleOutputAttribute(handle, attr, length, coord, &mut written);

	if result == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn fill_region(
	handle: HANDLE,
	start: COORD,
	length: u32,
	attr: WORD,
	reset_cursor: Option<COORD>,
) -> io::Result<()> {
	fill_console_output_character(handle, ' ' as u16, length, start)?;
	fill_console_output_attribute(handle, attr, length, start)?;

	if let Some(pos) = reset_cursor {
		set_cursor_pos(handle, pos)?;
	}

	Ok(())
}

unsafe fn set_console_screen_buffer_size(
	handle: HANDLE,
	size: COORD,
) -> io::Result<()> {
	if SetConsoleScreenBufferSize(handle, size) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn set_console_window_info(
	handle: HANDLE,
	absolute: bool,
	window: &SMALL_RECT,
) -> io::Result<()> {
	let flag = if absolute { 1 } else { 0 };
	if SetConsoleWindowInfo(handle, flag, window as *const _) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn get_largest_console_window_size(handle: HANDLE) -> io::Result<COORD> {
	let coord = GetLargestConsoleWindowSize(handle);
	if coord.X == 0 && coord.Y == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(coord)
}

unsafe fn write_console(handle: HANDLE, text: &str) -> io::Result<()> {
	let utf16: Vec<u16> = text.encode_utf16().collect();

	let mut written = 0;
	let result = WriteConsoleW(
		handle,
		utf16.as_ptr(),
		utf16.len() as u32,
		&mut written,
		std::ptr::null_mut(),
	);
	if result == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

unsafe fn create_file_a(
	file_name: *const i8,
	desired_access: u32,
	share_mode: u32,
	security_attributes: *mut c_void,
	creation_disposition: u32,
	flags_and_attributes: u32,
	template_file: HANDLE,
) -> io::Result<HANDLE> {
	let handle = unsafe {
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

	if handle == INVALID_HANDLE_VALUE || handle.is_null() {
		return Err(io::Error::last_os_error());
	}

	Ok(handle)
}

pub unsafe fn close_handle(handle: HANDLE) -> io::Result<()> {
	if CloseHandle(handle) == 0 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}
