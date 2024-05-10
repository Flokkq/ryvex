use std::cmp;
use std::io::{stdout, Write};

use crate::core::ui::{error::OverlayError, overlay::Overlay};

use super::MessageLevel;

pub enum MessageOverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct DecorativeMessageOverlay;

impl DecorativeMessageOverlay {
    pub const MAX_MESSAGE_WIDTH: u16 = 30;
    pub const MAX_MESSAGE_HEIGHT: u16 = 10;

    pub fn display_message(
        (cols, rows): (u16, u16),
        message: String,
        position: MessageOverlayPosition,
        level: MessageLevel,
    ) -> Result<(), OverlayError> {
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

        let mut handle = stdout().lock();
        Overlay::save_cursor_position(&mut handle);

        Self::render_message(&message, level, x, y, box_width, &mut handle)?;

        Overlay::restore_cursor_position(&mut handle);
        handle.flush()?;
        Ok(())
    }

    fn render_message(
        message: &str,
        level: MessageLevel,
        x: u16,
        y: u16,
        box_width: u16,
        handle: &mut std::io::StdoutLock,
    ) -> Result<(), OverlayError> {
        let (border_color, text_color) = level.to_color();
        write!(handle, "{}", border_color)?;

        write!(handle, "\x1b[{};{}H\x1b[K", x, y)?;
        write!(handle, "╭{:─<1$}╮", "", box_width as usize)?;

        let mut line_start = 0;
        let mut current_line = 0;
        while line_start < message.len()
            && current_line < Self::MAX_MESSAGE_HEIGHT
        {
            let line_end = cmp::min(
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

            write!(handle, "\x1b[{};{}H\x1b[K", x + 1 + current_line, y)?;
            write!(
                handle,
                "{border_color}│{text_color} {:>pad_start$}{}{:>pad_end$} {border_color}│",
                "",
                line,
                "",
                pad_start = pad_start,
                pad_end = pad_end,
            )?;

            line_start += line.len() + 1;
            current_line += 1;
        }

        write!(handle, "\x1b[{};{}H\x1b[K", x + 1 + current_line, y)?;
        write!(handle, "╰{:─<1$}╯", "", box_width as usize)?;

        write!(handle, "\x1b[0m")?;
        Ok(())
    }
}
