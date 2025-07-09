mod command;
mod cursor;
mod ffi;
mod terminal;

pub mod console;
pub mod handle;
pub mod source;

use crate::term::console::Handle as OtherHandle;
use ryvex_ui::graphics::Rect;
use std::{
	io,
	sync::OnceLock,
};

pub use console::ConsoleSettings;
pub use handle::ConsoleHandle as Handle;
pub use handle::ConsoleHandleSettings as HandleMode;
pub use source::WindowsEventSource as TargetEventSource;

static SUPPORTS_ANSI: OnceLock<bool> = OnceLock::new();

pub fn supports_ansi() -> bool {
	*SUPPORTS_ANSI.get_or_init(|| {
		if enable_vt_processing().is_err() {
			return false;
		}

		if let Ok(handle) = ffi::get_current_out_handle() {
			if let Ok(mode) = ffi::get_console_mode_from_handle(handle) {
				return mode & ffi::ENABLE_VIRTUAL_TERMINAL_PROCESSING != 0;
			}
		}
		false
	})
}

pub fn enable_vt_processing() -> io::Result<()> {
	let mask = ffi::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

	let handle = ffi::get_current_out_handle()?;
	let console_mode = ffi::get_console_mode_from_handle(handle)?;

	if console_mode & mask == 0 {
		let mode = console_mode | mask;
		ffi::set_console_mode(handle, mode)?;
	}

	Ok(())
}

pub fn get_terminal_size(handle: &handle::ConsoleHandle) -> io::Result<Rect> {
	let info = ffi::get_screen_buffer_info(handle.inner().clone())?;

	let width = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
	let height = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;

	Ok(Rect {
		x: 0,
		y: 0,
		width,
		height,
	})
}
