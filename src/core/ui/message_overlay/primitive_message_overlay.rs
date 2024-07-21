use std::{
    cmp,
    io::{stdin, stdout, Read, StdoutLock, Write},
};

use crate::core::{
    keys::keycode::KeyCode,
    ui::{error::OverlayError, overlay::Overlay},
};

use super::MessageLevel;

pub struct PrimitiveMessageOverlay;

impl PrimitiveMessageOverlay {
    pub fn display_message(
        (cols, rows): (u16, u16),
        message: String,
        level: MessageLevel,
    ) -> Result<(), OverlayError> {
        let mut handle = stdout().lock();

        Overlay::save_cursor_position(&mut handle);

        let (_, text_color) = level.to_color();
        let allow_multiline = matches!(level, MessageLevel::Error);
        let max_width = if allow_multiline { cols / 2 } else { cols };

        let lines = Self::split_message_into_lines(
            &message,
            max_width,
            allow_multiline,
        )?;

        let start_line = Self::calculate_start_line(rows, lines.len() as u16);

        Self::display_lines(&lines, start_line, text_color, &mut handle)?;

        if allow_multiline && lines.len() > 1 {
            Self::wait_for_user_action(
                start_line,
                lines.len() as u16,
                &mut handle,
            )?;
        }

        Overlay::restore_cursor_position(&mut handle);
        handle.flush()?;
        Ok(())
    }

    fn split_message_into_lines(
        message: &str,
        max_width: u16,
        allow_multiline: bool,
    ) -> Result<Vec<String>, OverlayError> {
        let mut lines = Vec::new();
        let mut line_start = 0;

        while line_start < message.len() {
            let line_end =
                cmp::min(line_start + max_width as usize, message.len());
            let mut line = message[line_start..line_end].to_string();

            if line.len() == max_width as usize && line_end < message.len() {
                if allow_multiline && line.contains(' ') {
                    if let Some(last_space) = line.rfind(' ') {
                        line = line[..last_space].to_string();
                        line_start += last_space + 1;
                    }
                } else {
                    lines.push(line);
                    break;
                }
            } else {
                line_start = line_end;
            }

            if !lines.contains(&line) {
                lines.push(line);
            }
        }
        Ok(lines)
    }

    fn calculate_start_line(total_rows: u16, num_lines: u16) -> u16 {
        cmp::max(1, total_rows - num_lines + 1)
    }

    fn display_lines(
        lines: &[String],
        start_line: u16,
        text_color: &str,
        handle: &mut StdoutLock,
    ) -> Result<(), OverlayError> {
        for (i, line) in lines.iter().enumerate() {
            write!(handle, "\x1B[{};1H", start_line + i as u16)?;
            write!(handle, "{}{}\x1b[0m", text_color, line)?;
            handle.flush()?;
        }
        Ok(())
    }

    fn wait_for_user_action(
        start_line: u16,
        num_lines: u16,
        handle: &mut StdoutLock,
    ) -> Result<(), OverlayError> {
        let mut stdin = stdin().lock();

        loop {
            let mut buffer = [0; 3];
            let bytes_read = stdin.read(&mut buffer)?;

            if let Some(key_code) = KeyCode::from_bytes(&buffer[..bytes_read]) {
                if [KeyCode::CarriageReturn, KeyCode::LineFeed, KeyCode::Esc]
                    .contains(&key_code)
                {
                    Overlay::remove_text(start_line, 1, num_lines)?;
                    Overlay::restore_cursor_position(handle);
                    break;
                }
            }
        }
        Ok(())
    }
}
