#[allow(non_camel_case_types)]
pub mod ffi;

use ffi::{
    GetConsoleScreenBufferInfo,
    CONSOLE_SCREEN_BUFFER_INFO,
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
use std::sync::{OnceLock, atomic::{AtomicU64, Ordering}};

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

unsafe fn get_screen_buffer_info(handle: HANDLE) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

    if GetConsoleScreenBufferInfo(handle, &mut info) == 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(info)
}
