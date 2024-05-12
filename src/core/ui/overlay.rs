use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::{self, StdoutLock, Write};
use std::os::unix::io::AsRawFd;

use super::message_overlay::{
    DecorativeMessageOverlay, PrimitiveMessageOverlay,
};
use super::{MessageLevel, MessageOverlayPosition};

pub struct Overlay;

impl Overlay {
    pub async fn display_decorative_message(
        message: String,
        position: MessageOverlayPosition,
        level: MessageLevel,
    ) {
        let _ = DecorativeMessageOverlay::display_message(
            Self::determine_window_size(),
            message,
            position,
            level,
        );
    }

    pub fn display_primitive_message(message: String, level: MessageLevel) {
        let _ = PrimitiveMessageOverlay::display_message(
            Self::determine_window_size(),
            message,
            level,
        );
    }

    fn determine_window_size() -> (u16, u16) {
        let mut wsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let stdout_fd = io::stdout().as_raw_fd();
        unsafe {
            ioctl(stdout_fd, TIOCGWINSZ, &mut wsize);
        }

        (wsize.ws_col, wsize.ws_row)
    }

    pub(super) fn save_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[s").unwrap();
    }

    pub(super) fn restore_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[u").unwrap();
    }
}
