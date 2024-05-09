use std::{
    io::{stdout, Write},
    usize,
};

use crate::core::ui::overlay::Overlay;

use super::MessageLevel;

pub struct PrimitiveMessageOverlay;

impl PrimitiveMessageOverlay {
    pub fn render_message(
        (cols, rows): (u16, u16),
        message: String,
        level: MessageLevel,
    ) {
        let mut handle = stdout().lock();

        Overlay::save_cursor_position(&mut handle);

        let (_, text_color) = level.to_color();

        let mut line_start = 0;
        let mut lines: Vec<String> = Vec::new();

        while line_start < message.len() {
            let line_end =
                std::cmp::min(line_start + (cols / 2) as usize, message.len());
            let mut line = message[line_start..line_end].to_string();

            if line.len() == (cols / 2) as usize && line_end < message.len() {
                if let Some(last_space) = line.rfind(' ') {
                    line = line[..last_space].to_string();
                }
            }

            lines.push(line);
            line_start += lines.last().unwrap().len() + 1;
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

        Overlay::restore_cursor_position(&mut handle);
        handle.flush().unwrap();
    }
}
