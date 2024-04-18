use std::{
    env::Args,
    io::{stdin, stdout, Read, StdoutLock, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    actions, configuration::Settings, error::Error, file_access::FileAccess,
    keys::keybind::KeyBind, telemetry::SingletonLogger,
};

pub struct OpenFile<'a> {
    path: PathBuf,
    buffer: &'a mut String,
}

pub fn run(
    file: &mut OpenFile,
    keybinds: Vec<KeyBind>,
    stdout: &mut StdoutLock,
) -> Result<(), Error> {
    stdout
        .write_all(file.buffer.as_bytes())
        .map_err(|err| Error::Io(err))?;

    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut buffer = [0; 1];

    loop {
        handle
            .read_exact(&mut buffer)
            .map_err(|err| Error::Io(err))?;

        if !process_keypress(buffer[0], &keybinds)
            .map_err(|err| Error::Action(err))?
        {
            match buffer[0] {
                b'\x08' | b'\x7f' => {
                    if !file.buffer.is_empty() {
                        file.buffer.pop();
                        stdout.write_all(b"\x08 \x08")?;
                    }
                }
                b'\n' | b'\r' => {
                    file.buffer.push('\r');
                    file.buffer.push('\n');
                    stdout.write_all(b"\r\n")?;
                }
                _ => {
                    if buffer[0].is_ascii_alphanumeric()
                        || buffer[0].is_ascii_punctuation()
                        || buffer[0] == b' '
                    {
                        file.buffer.push(buffer[0] as char);
                        stdout.write_all(&buffer)?;
                    }
                }
            }
            stdout.flush().map_err(|_| Error::Unexpected)?;
        }
    }
}

fn process_keypress(
    key: u8,
    keybinds: &Vec<KeyBind>,
) -> Result<bool, actions::error::ActionError> {
    for keybind in keybinds {
        if keybind.keys.iter().any(|k| k.key == key.to_string()) {
            let action = &keybind.on_activate;
            action()?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn build(configuration: Settings, args: &mut Args) -> Result<(), Error> {
    let filename = args.nth(1);

    match filename {
        Some(file) => {
            let mut stdout = stdout().lock();
            let mut buffer = String::new();

            let path =
                PathBuf::from_str(&file).map_err(|_| Error::Unexpected)?;
            FileAccess::read_from_file_if_exists(&path, &mut buffer)?;

            let mut open_file = OpenFile {
                path,
                buffer: &mut buffer,
            };

            run(&mut open_file, configuration.keybinds, &mut stdout)?;
            Ok(())
        }
        None => {
            let logger = SingletonLogger::get_instance();
            logger.error("No filename provided when starting");

            return Err(Error::Unexpected);
        }
    }
}
