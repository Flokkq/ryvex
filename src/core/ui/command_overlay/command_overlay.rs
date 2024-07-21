use std::{
    io::{stdin, stdout, Read, StdoutLock, Write},
    process::{Command as OSCommand, Stdio},
};

use crate::core::{
    actions::{action::ActionResult, error::ActionError},
    buffer::Buffer,
    command::Command,
    keys::keycode::KeyCode,
    ui::{error::OverlayError, overlay::Overlay},
};

pub struct CommandOverlay;

impl CommandOverlay {
    pub fn display_overlay(
        (_cols, rows): (u16, u16),
        custom_commands: &Vec<Command>,
        input: &str,
    ) -> Result<ActionResult, OverlayError> {
        let mut handle: StdoutLock = stdout().lock();
        let mut buf = Buffer::new(input.to_string());
        let mut result = ActionResult::Continue;

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
                    result =
                        Self::execute_command(custom_commands, buf.content())
                            .map_err(OverlayError::CommandError)?;
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
        Ok(result)
    }

    fn execute_command(
        custom_commands: &Vec<Command>,
        input: &String,
    ) -> Result<ActionResult, ActionError> {
        let content = input.strip_prefix(":").unwrap_or("");

        if let Some(first_char) = content.chars().nth(0) {
            if first_char == '!' {
                let command = &content[1..];
                let parts: Vec<&str> = command.split_whitespace().collect();

                if !parts.is_empty() {
                    let program = parts[0];
                    let args = &parts[1..];

                    let status = OSCommand::new(program)
                        .args(args)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .status()
                        .map_err(|_| ActionError::ExecutionFailed)?;

                    if !status.success() {
                        return Err(ActionError::ExecutionFailed);
                    }
                }
            } else {
                if let Some(custom_command) =
                    custom_commands.iter().find(|cmd| cmd.alias == content)
                {
                    // display_overlay_message returns an ActionResult...
                    let res = (custom_command.callback)()?;
                    return Ok(res);
                } else {
                    Overlay::display_primitive_message(
                        format!("`{}` is not a valid command", input),
                        crate::core::ui::MessageLevel::Error,
                    );

                    return Err(ActionError::Unexpected);
                }
            }
        } else {
            Overlay::display_primitive_message(
                format!("`{}` is not a valid command", input),
                crate::core::ui::MessageLevel::Error,
            );

            return Err(ActionError::Unexpected);
        }

        Ok(ActionResult::Continue)
    }
}
