use std::{
    env::Args,
    io::{stdin, stdout, Read, StdoutLock, Write},
    path::PathBuf,
};

use crate::{
    configuration::Settings,
    core::{
        actions::error::ActionError,
        error::Error,
        keys::{
            key::KeyType,
            keybind::{ActionResult, KeyBind},
            keycode::{EscapeSequence, KeyCode},
        },
        layers::layer::{TerminalLayer, VisualLayer},
        open_file::OpenFile,
        state::{get_global_state, set_open_file},
        ui::{overlay::Overlay, MessageLevel, MessageOverlayPosition},
    },
    telemetry::SingletonLogger,
};

pub fn run(
    keybinds: Vec<KeyBind>,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    display_file_buffer(stdout)?;

    loop {
        let global_state = get_global_state();

        let mut state_guard =
            global_state.get_state().map_err(|_| Error::Unexpected)?;
        let file = state_guard.file.as_mut().ok_or(Error::Unexpected)?;

        let mut handle = stdin().lock();
        let mut buffer = [0; 3];

        let bytes_read = handle.read(&mut buffer)?;
        if bytes_read == 0 {
            continue;
        }

        drop(handle);

        let key_code = KeyCode::from_bytes(&buffer[..bytes_read]);
        match key_code {
            Some(KeyCode::Esc) => {
                file.buffer.change_layer(TerminalLayer::Normal);
                Overlay::display_primitive_message(
                    "NORMAL".to_string(),
                    MessageLevel::Info,
                );
            }
            Some(KeyCode::LowerI) => {
                if &TerminalLayer::Insert == file.buffer.layer() {
                    drop(state_guard);
                    process_buffer(&KeyCode::LowerI, stdout)?;
                } else if &TerminalLayer::Replace == file.buffer.layer() {
                } else {
                    file.buffer.change_layer(TerminalLayer::Insert);

                    Overlay::display_primitive_message(
                        "INSERT".to_string(),
                        MessageLevel::Info,
                    );
                }
            }
            Some(KeyCode::LowerV) => {
                file.buffer
                    .change_layer(TerminalLayer::Visual(VisualLayer::Block));
                Overlay::display_primitive_message(
                    "VISUAL".to_string(),
                    MessageLevel::Info,
                );
            }
            Some(KeyCode::UpperV) => {
                file.buffer
                    .change_layer(TerminalLayer::Visual(VisualLayer::Line));

                Overlay::display_primitive_message(
                    "VISUAL-LINE".to_string(),
                    MessageLevel::Info,
                );
            }
            Some(KeyCode::UpperR) => {
                file.buffer.change_layer(TerminalLayer::Replace);

                Overlay::display_primitive_message(
                    "REPLACE".to_string(),
                    MessageLevel::Info,
                );
            }
            Some(KeyCode::EscapeSequence(seq)) => {
                drop(state_guard);
                handle_escape_sequence(seq, stdout)?;
            }
            Some(code) => match file.buffer.layer() {
                TerminalLayer::Insert => {
                    drop(state_guard);
                    process_buffer(&code, stdout)?;
                }
                TerminalLayer::Replace => {
                    todo!()
                }
                _ => {
                    drop(state_guard);

                    let action_result = process_keypress(&code, &keybinds)?;
                    if matches!(action_result, ActionResult::Exit) {
                        return Ok(());
                    }
                }
            },
            _ => return Err(Error::Unexpected),
        }
    }
}

fn process_buffer(
    key_code: &KeyCode,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    let global_state = get_global_state();

    let mut state_guard =
        global_state.get_state().map_err(|_| Error::Unexpected)?;
    let file = state_guard.file.as_mut().ok_or(Error::Unexpected)?;

    match key_code {
        KeyCode::Backspace | KeyCode::Del => {
            if !file.is_empty() {
                file.delete();
            }
        }
        KeyCode::LineFeed | KeyCode::CarriageReturn => {
            file.insert_newline();
        }
        kc if kc.to_key_type() != KeyType::Unknown
            && kc.to_key_type() != KeyType::Control =>
        {
            if let Some(char) = kc.as_str().as_bytes().get(0) {
                file.insert(*char as char);
            }
        }
        _ => return Ok(()),
    }

    file.redraw(stdout)?;

    Ok(())
}

fn process_keypress(
    key: &KeyCode,
    keybinds: &[KeyBind],
) -> Result<ActionResult, ActionError> {
    keybinds
        .iter()
        .find_map(|keybind| {
            if keybind.key.key_code == *key {
                if let Some(callback) = keybind.callback {
                    Some(callback())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(Ok(ActionResult::Continue))
}

fn handle_escape_sequence(
    seq: EscapeSequence,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    let global_state = get_global_state();
    let mut state_guard =
        global_state.get_state().map_err(|_| Error::Unexpected)?;
    let file = state_guard.file.as_mut().ok_or(Error::Unexpected)?;

    file.move_cursor(seq);

    file.redraw(stdout)?;
    Ok(())
}

fn display_file_buffer(stdout: &mut StdoutLock) -> Result<(), Error> {
    let global_state = get_global_state();
    let state_guard =
        global_state.get_state().map_err(|_| Error::Unexpected)?;

    let file = state_guard.file.as_ref().ok_or(Error::Unexpected)?;
    file.redraw(stdout)?;
    stdout.flush().map_err(|_| Error::Unexpected)?;

    Ok(())
}

pub fn build(configuration: Settings, args: &mut Args) -> Result<(), Error> {
    let filename = args.nth(1);

    match filename {
        Some(file) => {
            let mut stdout = stdout().lock();

            let open_file = OpenFile::open(PathBuf::from(file))
                .map_err(|_| Error::Unexpected)?;

            set_open_file(open_file);

            run(configuration.keybinds, &mut stdout)?;
            Ok(())
        }
        None => {
            let logger = SingletonLogger::get_instance();
            logger.error("No filename provided when starting");

            return Err(Error::Unexpected);
        }
    }
}
