use std::{
    io::{stdin, stdout, Read, Write},
    usize,
};

use crate::core::{
    keys::keycode::KeyCode,
    ui::{error::OverlayError, overlay::Overlay},
};

use super::MessageLevel;

pub struct PrimitiveMessageOverlay;

impl PrimitiveMessageOverlay {
    pub fn render_message(
        (cols, rows): (u16, u16),
        message: String,
        level: MessageLevel,
    ) -> Result<(), OverlayError> {
        let mut handle = stdout().lock();

        Overlay::save_cursor_position(&mut handle);

        let (_, text_color) = level.to_color();

        let mut line_start = 0;
        let mut lines: Vec<String> = Vec::new();

        let allow_multiline = matches!(level, MessageLevel::Error);
        let max_width = if allow_multiline { cols / 2 } else { cols };

        while line_start < message.len() {
            let line_end =
                std::cmp::min(line_start + max_width as usize, message.len());
            let mut line = message[line_start..line_end].to_string();

            if line.len() == max_width as usize && line_end < message.len() {
                if allow_multiline {
                    if let Some(last_space) = line.rfind(' ') {
                        line = line[..last_space].to_string();
                    }
                } else {
                    line = message[line_start..max_width as usize].to_string();
                    lines.push(line);
                    break;
                }
            }

            lines.push(line);
            line_start += lines.last().unwrap().len() + 1;
            if !allow_multiline {
                break;
            }
        }

        let start_line = if lines.len() <= rows as usize {
            rows - lines.len() as u16 + 1
        } else {
            1
        };

        for (i, line) in lines.iter().enumerate() {
            write!(handle, "\x1B[{};1H", start_line + i as u16).unwrap();
            write!(handle, "{}{}\x1b[0m", text_color, line).unwrap();
        }

        handle.flush().unwrap();
        if lines.len() > 1 && allow_multiline {
            let mut stdin = stdin().lock();

            loop {
                let mut buffer = [0; 3];
                match stdin.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        if let Some(key_code) =
                            KeyCode::from_bytes(&buffer[..bytes_read])
                        {
                            if key_code == KeyCode::CarriageReturn
                                || key_code == KeyCode::LineFeed
                                || key_code == KeyCode::Esc
                            {
                                break;
                            }
                        }
                    }
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        return Err(OverlayError::Io(e));
                    }
                }
            }
        }

        Overlay::restore_cursor_position(&mut handle);
        handle.flush().unwrap();

        Ok(())
    }
}
