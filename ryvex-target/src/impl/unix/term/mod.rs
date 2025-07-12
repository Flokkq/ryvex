mod command;

pub mod fd;
pub mod source;
pub mod termios;

use super::{
	ffi,
	target::os::TIOCGWINSZ,
};
use crate::std::error::IoError;
use crate::std::Result;
use ryvex_ui::graphics::Rect;

pub use fd::TtyFd as Handle;
pub use fd::TtyFdSettings as HandleMode;
pub use source::UnixEventSource as TargetEventSource;
pub use termios::Termios as ConsoleSettings;

/// since we only ever open `stdin` or `stdout` ANSI escape codes are always
/// supported
pub(crate) fn supports_ansi() -> bool {
	true
}

pub fn get_terminal_size(fd: &fd::TtyFd) -> Result<Rect> {
	// unwrap is safe because we know that `TIOCGWINSZ` always fits into a
	// `c_ulong`
	let winsize = ffi::ioctl(fd, TIOCGWINSZ.try_into().unwrap())
		.map_err(IoError::from)?;

	Ok(Rect {
		width:  winsize.ws_col,
		height: winsize.ws_row,
		x:      0,
		y:      0,
	})
}
