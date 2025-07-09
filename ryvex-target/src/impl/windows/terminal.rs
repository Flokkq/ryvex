use super::ffi;
use crate::term::command::terminal::ClearType;
use std::io;

pub fn scroll_up(count: u16) -> io::Result<()> {
	scroll_to(count as i16)
}

pub fn scroll_down(count: u16) -> io::Result<()> {
	scroll_to(-(count as i16))
}

pub fn clear(clear_type: ClearType) -> io::Result<()> {
	let handle = ffi::get_current_out_handle()?;
	let csbi = ffi::get_screen_buffer_info(handle)?;
	let sr = csbi.srWindow;
	let attr = csbi.wAttributes;
	let width = (sr.Right - sr.Left + 1) as u32;
	let height = (sr.Bottom - sr.Top + 1) as u32;
	let cursor = csbi.dwCursorPosition;

	match clear_type {
		ClearType::All | ClearType::Purge => {
			let origin = ffi::COORD {
				X: sr.Left,
				Y: sr.Top,
			};

			fill_region(handle, origin, width * height, attr, Some(origin))
		}
		ClearType::FromCursorDown => {
			let consumed = (cursor.Y as u32 - sr.Top as u32) * width +
				(cursor.X as u32 - sr.Left as u32);
			let remaining = width * height - consumed;

			fill_region(handle, cursor, remaining, attr, None)
		}
		ClearType::FromCursorUp => {
			let length = (cursor.Y as u32 - sr.Top as u32 + 1) * width;
			let start = ffi::COORD {
				X: sr.Left,
				Y: sr.Top,
			};

			fill_region(handle, start, length, attr, None)
		}
		ClearType::CurrentLine => {
			let start = ffi::COORD {
				X: sr.Left,
				Y: cursor.Y,
			};

			fill_region(handle, start, width, attr, None)
		}
		ClearType::UntilNewLine => {
			let remaining = width - (cursor.X as u32 - sr.Left as u32);

			fill_region(handle, cursor, remaining, attr, None)
		}
	}
}

fn scroll_to(count: i16) -> io::Result<()> {
	let handle = ffi::get_current_out_handle()?;
	let csbi = ffi::get_screen_buffer_info(handle)?;
	let rect = csbi.srWindow;

	let dest = ffi::COORD {
		X: rect.Left,
		Y: rect.Top as i16 + count,
	};

	let ch = ffi::CHAR_INFO_Char {
		UnicodeChar: b' ' as u16,
	};

	let fill = ffi::CHAR_INFO {
		Char:       ch,
		Attributes: csbi.wAttributes,
	};

	ffi::scroll_screen_buffer(handle, Some(&rect), None, dest, fill)
}

pub fn set_size(width: u16, height: u16) -> io::Result<()> {
	if width < 1 || height < 1 {
		return Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"width and height must be at least 1",
		));
	}

	let handle = ffi::get_current_out_handle()?;
	let csbi = ffi::get_screen_buffer_info(handle)?;
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
		ffi::set_console_screen_buffer_size(handle, new_buf)?;
	}

	let mut new_win = win;
	new_win.Right = win.Left + w - 1;
	new_win.Bottom = win.Top + h - 1;
	ffi::set_console_window_info(handle, true, &new_win)?;

	if resized_buffer {
		ffi::set_console_screen_buffer_size(handle, orig_buf)?;
	}

	let max_win = ffi::get_largest_console_window_size(handle)?;
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
	let handle = ffi::get_current_out_handle()?;

	ffi::write_console(handle, text)
}

fn fill_region(
	handle: ffi::HANDLE,
	start: ffi::COORD,
	length: u32,
	attr: ffi::WORD,
	reset_cursor: Option<ffi::COORD>,
) -> io::Result<()> {
	ffi::fill_console_output_character(handle, ' ' as u16, length, start)?;
	ffi::fill_console_output_attribute(handle, attr, length, start)?;

	if let Some(pos) = reset_cursor {
		ffi::set_cursor_position(handle, pos)?;
	}

	Ok(())
}
