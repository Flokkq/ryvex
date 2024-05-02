use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

pub struct Overlay;

impl Overlay {
    pub fn show(message: String, position: OverlayPosition) {
        let (cols, rows) = Self::determine_window_size();
        let padding = 2;

        // Message dimensions including box borders
        let message_length = message.chars().count() as u16;
        let box_width = message_length + 2; // Padding for the box sides

        let (x, y) = match position {
            OverlayPosition::TopLeft => (1 + padding, padding),
            OverlayPosition::TopRight => {
                (1 + padding, cols.saturating_sub(box_width + padding))
            }
            OverlayPosition::BottomLeft => {
                (rows.saturating_sub(3 + padding), padding)
            }
            OverlayPosition::BottomRight => (
                rows.saturating_sub(3 + padding),
                cols.saturating_sub(box_width + padding),
            ),
            OverlayPosition::MidLeft => (rows / 2, padding + cols / 4),
            OverlayPosition::MidRight => (
                rows / 2,
                cols.saturating_sub(box_width + padding) - cols / 4,
            ),
            OverlayPosition::Mid => (rows / 2, cols / 2 - message_length / 2),
        };

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        // Save the cursor position
        handle.write_all(b"\x1b[s").unwrap();

        // Draw the top border
        write!(handle, "\x1b[{};{}H\x1b[K", x, y).unwrap();
        write!(handle, "╭{:─<1$}╮", "", box_width as usize).unwrap(); // Top border of the box

        // Draw the message content
        write!(handle, "\x1b[{};{}H\x1b[K", x + 1, y).unwrap();
        write!(handle, "│ {} │", message).unwrap(); // Message content

        // Draw the bottom border
        write!(handle, "\x1b[{};{}H\x1b[K", x + 2, y).unwrap();
        write!(handle, "╰{:─<1$}╯", "", box_width as usize).unwrap(); // Bottom border of the box

        // Restore the cursor position
        handle.write_all(b"\x1b[u").unwrap();
        handle.flush().unwrap();
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

pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    MidLeft,
    MidRight,
    Mid,
}
