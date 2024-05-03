use std::io::{stdout, StdoutLock, Write};

use super::MessageLevel;

pub struct PrimitiveMessageOverlay;

impl PrimitiveMessageOverlay {
    pub fn render_message(
        (cols, rows): (u16, u16),
        message: String,
        level: MessageLevel,
    ) {
        let stdout = stdout();
        let mut handle = stdout.lock();

        Self::save_cursor_position(&mut handle);

        let (_, text_color) = level.to_color();

        let mut final_message = message;
        if final_message.len() as u16 > cols {
            final_message =
                final_message.chars().take((cols - 3) as usize).collect();
            final_message.push_str("...");
        }

        write!(handle, "\x1B[{};1H", rows).unwrap();
        write!(handle, "{}{}\x1b[0m", text_color, final_message).unwrap();

        Self::restore_cursor_position(&mut handle);
        handle.flush().unwrap();
    }

    fn save_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[s").unwrap();
    }

    fn restore_cursor_position(handle: &mut StdoutLock) {
        handle.write_all(b"\x1b[u").unwrap();
    }
}
