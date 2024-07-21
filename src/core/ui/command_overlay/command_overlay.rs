use std::{
    io::{stdin, stdout, Read, StdoutLock, Write},
    process::{Command, Stdio},
};

use crate::core::{
    buffer::Buffer,
    keys::keycode::KeyCode,
    ui::{error::OverlayError, overlay::Overlay},
};

pub struct CommandOverlay;

impl CommandOverlay {
    pub fn display_overlay(
        (_cols, rows): (u16, u16),
        input: &str,
    ) -> Result<(), OverlayError> {
        let mut handle: StdoutLock = stdout().lock();
        let mut buf = Buffer::new(input.to_string());

        Overlay::save_cursor_position(&mut handle);

        write!(handle, "\x1B[{};1H", rows)?;
        write!(handle, "{}\x1b[0m", input)?;
        handle.flush()?;

        let mut stdin = stdin().lock();

        loop {
            let buffer = [0; 3];
            let mut buffer = buffer;
            let bytes_read = stdin.read(&mut buffer)?;

            match KeyCode::from_bytes(&buffer[..bytes_read]) {
                Some(KeyCode::Esc) => {
                    Overlay::remove_text(rows, 1, 1)?;
                    break;
                }
                Some(KeyCode::LineFeed) | Some(KeyCode::CarriageReturn) => {
                    let _ = Self::execute_command(buf.content());
                    break;
                }
                Some(code) => match code {
                    KeyCode::Backspace | KeyCode::Del => {
                        if !buf.is_empty() {
                            buf.delete();
                        }
                    }
                    _ => {
                        if let Some(char) = code.as_str().as_bytes().first() {
                            buf.insert(*char as char);
                        }
                    }
                },
                None => {
                    Overlay::remove_text(rows, 1, 1)?;
                    break;
                }
            }

            write!(handle, "\x1B[{};1H", rows)?;
            write!(handle, "{}\x1b[0m", buf.content())?;
            handle.flush()?;
        }

        Overlay::restore_cursor_position(&mut handle);
        handle.flush()?;
        Ok(())
    }

    fn execute_command(input: &String) -> Result<(), OverlayError> {
        let content = input.strip_prefix(":").unwrap_or("");
        if let Some(first_char) = content.chars().nth(0) {
            if let Some(KeyCode::Exclamation) =
                KeyCode::from_ascii(first_char as u8)
            {
                let command = &content[1..];
                let parts: Vec<&str> = command.split_whitespace().collect();

                if !parts.is_empty() {
                    let program = parts[0];
                    let args = &parts[1..];

                    let status = Command::new(program)
                        .args(args)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .status()
                        .map_err(OverlayError::CommandExecutionError)?;

                    if !status.success() {
                        return Err(OverlayError::CommandExecutionError(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Command did not execute successfully",
                            ),
                        ));
                    }
                }
            } else {
                Overlay::display_primitive_message(
                    format!("`{}` is not a valid command", input),
                    crate::core::ui::MessageLevel::Error,
                );

                return Err(OverlayError::Unexpected);
            }
        } else {
            Overlay::display_primitive_message(
                format!("`{}` is not a valid command", input),
                crate::core::ui::MessageLevel::Error,
            );

            return Err(OverlayError::Unexpected);
        }

        Ok(())
    }
}
