use super::ffi;
use std::ffi::c_void;
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

static SAVED_CURSOR_POS: AtomicU64 = AtomicU64::new(u64::MAX);

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

pub fn move_to(x: i16, y: i16) -> io::Result<()> {
	if x < 0 {
		return Err(io::Error::other(format!(
			"cursor position out of range - X: {x}"
		)));
	}

	if y < 0 {
		return Err(io::Error::other(format!(
			"cursor position out of range - Y: {y}"
		)));
	}

	let point = ffi::COORD { X: x, Y: y };
	let handle = ffi::get_current_out_handle()?;

	ffi::set_cursor_position(handle, point)
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
	let handle = ffi::get_current_out_handle()?;
	let info = ffi::get_screen_buffer_info(handle)?;

	Ok((info.dwCursorPosition.X, info.dwCursorPosition.Y))
}

pub fn show_cursor(show: bool) -> io::Result<()> {
	let handle = ffi::get_current_out_handle()?;

	let mut info = ffi::get_cursor_info(handle)?;
	info.bVisible = if show { 1 } else { 0 };

	ffi::set_cursor_info(handle, &info)
}
