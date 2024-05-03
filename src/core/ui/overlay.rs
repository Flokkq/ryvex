use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::{self};
use std::os::unix::io::AsRawFd;

use super::message_overlay::MessageOverlay;
use super::{MessageLevel, MessageOverlayPosition};

pub struct Overlay;

impl Overlay {
    pub fn render_message(
        message: String,
        position: MessageOverlayPosition,
        level: MessageLevel,
    ) {
        MessageOverlay::render_message(
            Self::determine_window_size(),
            message,
            position,
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
}
