use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::{self, StdoutLock, Write};
use std::os::unix::io::AsRawFd;

pub struct Overlay;

impl Overlay {
    const MAX_MESSAGE_WIDTH: u16 = 30;
    const MAX_MESSAGE_HEIGHT: u16 = 10;

    pub fn show_message(
        message: String,
        position: MessageOverlayPosition,
        level: MessageLevel,
    ) {
        let (cols, rows) = Self::determine_window_size();
        let horizontal_padding = 2;
        let vertical_padding = 1;

        let box_width = Self::MAX_MESSAGE_WIDTH + horizontal_padding;
        let (x, y) = match position {
            MessageOverlayPosition::TopLeft => {
                (1 + vertical_padding, horizontal_padding)
            }
            MessageOverlayPosition::TopRight => (
                1 + vertical_padding,
                cols.saturating_sub(box_width + horizontal_padding),
            ),
            MessageOverlayPosition::BottomLeft => (
                rows.saturating_sub(
                    Self::MAX_MESSAGE_HEIGHT + vertical_padding + 3,
                ),
                horizontal_padding,
            ),
            MessageOverlayPosition::BottomRight => (
                rows.saturating_sub(
                    Self::MAX_MESSAGE_HEIGHT + vertical_padding + 3,
                ),
                cols.saturating_sub(box_width + horizontal_padding),
            ),
        };

        Self::display_message(&message, level, x, y, box_width);
    }

    fn display_message(
        message: &str,
        level: MessageLevel,
        x: u16,
        y: u16,
        box_width: u16,
    ) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        Self::save_cursor_position(&mut handle);

        let (border_color, text_color) = level.to_color();
        write!(handle, "{}", border_color).unwrap();

        write!(handle, "\x1b[{};{}H\x1b[K", x, y).unwrap();
        write!(handle, "╭{:─<1$}╮", "", box_width as usize).unwrap();

        let mut line_start = 0;
        let mut current_line = 0;
        while line_start < message.len()
            && current_line < Self::MAX_MESSAGE_HEIGHT
        {
            let line_end = std::cmp::min(
                line_start + Self::MAX_MESSAGE_WIDTH as usize,
                message.len(),
            );
            let mut line = &message[line_start..line_end];

            if line.len() == Self::MAX_MESSAGE_WIDTH as usize
                && line_end < message.len()
            {
                if let Some(last_space) = line.rfind(' ') {
                    line = &line[..last_space];
                }
            }

            let total_padding = box_width as usize - line.len() - 2;
            let pad_start = total_padding / 2;
            let pad_end = total_padding - pad_start;

            write!(handle, "\x1b[{};{}H\x1b[K", x + 1 + current_line, y)
                .unwrap();
            write!(
                handle,
                "{border_color}│{text_color} {:>pad_start$}{}{:>pad_end$} {border_color}│",
                "",
                line,
                "",
                pad_start = pad_start,
                pad_end = pad_end,
            )
            .unwrap();

            line_start += line.len() + 1;
            current_line += 1;
        }

        write!(handle, "\x1b[{};{}H\x1b[K", x + 1 + current_line, y).unwrap();
        write!(handle, "╰{:─<1$}╯", "", box_width as usize).unwrap();

        write!(handle, "\x1b[0m").unwrap();

        Self::restore_cursor_position(&mut handle);
        handle.flush().unwrap();
    }

    fn save_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[s").unwrap();
    }

    fn restore_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[u").unwrap();
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

pub enum MessageOverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

impl MessageLevel {
    pub fn to_color(&self) -> (&str, &str) {
        let (border_color, text_color) = match self {
            MessageLevel::Info => ("\x1b[34m", "\x1b[0m"), // blue border, default text color
            MessageLevel::Warning => ("\x1b[33m", "\x1b[0m"), // yellow border, default text color
            MessageLevel::Error => ("\x1b[31m", "\x1b[0m"), // red border, default text color
        };

        return (border_color, text_color);
    }
}
