use std::{
    env::Args,
    io::{stdin, stdout, Read, StdoutLock, Write},
    path::PathBuf,
};

use crate::{
    actions::error::ActionError,
    configuration::Settings,
    error::Error,
    file_access::FileAccess,
    keys::keybind::{ActionResult, KeyBind},
    state::{get_global_state, set_open_file},
    telemetry::SingletonLogger,
};

pub struct OpenFile {
    pub path: PathBuf,
    pub buffer: String,
}

pub fn run(
    keybinds: Vec<KeyBind>,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    let global_state = get_global_state();
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut buffer = [0; 1];

    {
        let state_guard =
            global_state.get_state().map_err(|_| Error::Unexpected)?;
        if let Some(file) = &state_guard.file {
            stdout.write_all(file.buffer.as_bytes())?;
            stdout.flush().map_err(|_| Error::Unexpected)?;
        }
    }

    loop {
        handle
            .read_exact(&mut buffer)
            .map_err(|err| Error::Io(err))?;
        let mut output_update: Option<Vec<u8>> = None;

        {
            let mut state_guard =
                global_state.get_state().map_err(|_| Error::Unexpected)?;
            let file = match &mut state_guard.file {
                Some(file) => file,
                None => return Err(Error::Unexpected),
            };

            if buffer[0] == b'\x08' || buffer[0] == b'\x7f' {
                if !file.buffer.is_empty() {
                    file.buffer.pop();
                    output_update = Some(vec![b'\x08', b' ', b'\x08']);
                }
            } else if buffer[0] == b'\n' || buffer[0] == b'\r' {
                file.buffer.push('\r');
                file.buffer.push('\n');
                output_update = Some(vec![b'\r', b'\n']);
            } else if buffer[0].is_ascii_alphanumeric()
                || buffer[0].is_ascii_punctuation()
                || buffer[0] == b' '
            {
                file.buffer.push(buffer[0] as char);
                output_update = Some(vec![buffer[0]]);
            }
        }

        if let Some(data) = output_update {
            stdout.write_all(&data)?;
            stdout.flush().map_err(|_| Error::Unexpected)?;
        }

        let action_result = process_keypress(buffer[0], &keybinds)?;
        if matches!(action_result, ActionResult::Exit) {
            return Ok(());
        }
    }
}

fn process_keypress(
    key: u8,
    keybinds: &Vec<KeyBind>,
) -> Result<ActionResult, ActionError> {
    for keybind in keybinds {
        if keybind.keys.iter().any(|k| k.key == key.to_string()) {
            let action_result = (keybind.on_activate)()?;
            return Ok(action_result);
        }
    }
    Ok(ActionResult::Continue)
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

impl OpenFile {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let mut buffer = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut buffer)?;

        Ok(OpenFile { path, buffer })
    }
}
