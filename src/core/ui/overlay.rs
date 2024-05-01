use std::os::fd::AsRawFd;

use libc::{ioctl, winsize, TIOCGWINSZ};

pub struct Overlay;

impl Overlay {
    pub fn show_top_right(message: String) {
        let (cols, _) = Self::determine_window_size();
        let padding = 2; // Define padding, could be a percentage of `cols`
        let start_pos =
            cols.saturating_sub(message.chars().count() as u16 + padding);

        // Save the cursor position
        print!("\x1b[s");

        // Move to top right, clear the line, and print the message
        println!("\x1b[{};{}H\x1b[K{}", 1, start_pos, message);

        // Restore the cursor position
        print!("\x1b[u");
    }

    fn determine_window_size() -> (u16, u16) {
        let mut wsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let stdout_fd = std::io::stdout().as_raw_fd();
        unsafe {
            ioctl(stdout_fd, TIOCGWINSZ, &mut wsize);
        }

        (wsize.ws_col, wsize.ws_row)
    }
}
