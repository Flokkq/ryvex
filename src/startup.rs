use std::{
    env::Args,
    io::{stdin, stdout, Read, StdoutLock, Write},
    path::PathBuf,
};

use crate::{
    actions::error::ActionError,
    configuration::Settings,
    error::Error,
    keys::{
        key::KeyType,
        keybind::{ActionResult, KeyBind},
        keycode::KeyCode,
    },
    open_file::OpenFile,
    state::{get_global_state, set_open_file},
    telemetry::SingletonLogger,
};

pub fn run(
    keybinds: Vec<KeyBind>,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut buffer = [0; 1];

    loop {
        handle.read_exact(&mut buffer).map_err(Error::Io)?;
        let key_code = KeyCode::from_ascii(buffer[0]);

        let action_result = process_keypress(buffer[0], &keybinds)?;
        if matches!(action_result, ActionResult::Exit) {
            return Ok(());
        }

        process_buffer(key_code, stdout)?;
    }
}

fn process_buffer(
    key_code: Option<KeyCode>,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    let global_state = get_global_state();
    let state_guard =
        global_state.get_state().map_err(|_| Error::Unexpected)?;
    let mut output_update = None;

    {
        let file = state_guard.file.as_ref().ok_or(Error::Unexpected)?;
        stdout.write_all(file.buffer.as_bytes())?;
        stdout.flush().map_err(|_| Error::Unexpected)?;
    }

    let mut state_guard =
        global_state.get_state().map_err(|_| Error::Unexpected)?;
    let file = state_guard.file.as_mut().ok_or(Error::Unexpected)?;

    match key_code {
        Some(KeyCode::Backspace) | Some(KeyCode::Del) => {
            if !file.buffer.is_empty() {
                file.buffer.pop();
                output_update = Some(vec![b'\x08', b' ', b'\x08']);
            }
        }
        Some(KeyCode::LineFeed) | Some(KeyCode::CarriageReturn) => {
            file.buffer.push('\r');
            file.buffer.push('\n');
            output_update = Some(vec![b'\r', b'\n']);
        }
        Some(kc)
            if kc.to_key_type() != KeyType::Unknown
                && kc.to_key_type() != KeyType::Control =>
        {
            if let Some(char) = kc.to_character().as_bytes().get(0) {
                file.buffer.push(*char as char);
                output_update = Some(vec![*char]);
            }
        }
        _ => {}
    }

    if let Some(data) = output_update {
        stdout.write_all(&data)?;
        stdout.flush().map_err(|_| Error::Unexpected)?;
    }

    Ok(())
}

fn process_keypress(
    key: u8,
    keybinds: &[KeyBind],
) -> Result<ActionResult, ActionError> {
    keybinds
        .iter()
        .find_map(|keybind| {
            if keybind.keys.iter().any(|k| {
                *k.key_code.to_character().as_bytes().get(0).unwrap_or(&0)
                    == key
            }) {
                Some((keybind.on_activate)())
            } else {
                None
            }
        })
        .unwrap_or(Ok(ActionResult::Continue))
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
