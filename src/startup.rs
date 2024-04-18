use std::{
    env::Args,
    io::{stdin, stdout, Read, StdoutLock, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    actions, configuration::Settings, file_access::FileAccess,
    keys::keybind::KeyBind,
};

pub struct OpenFile<'a> {
    path: PathBuf,
    buffer: &'a mut String,
}

pub fn run(
    file: &mut OpenFile,
    keybinds: Vec<KeyBind>,
    stdout: &mut StdoutLock,
) -> Result<(), actions::error::Error> {
    stdout.write_all(file.buffer.as_bytes()).unwrap();
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut buffer = [0; 1];

    loop {
        handle.read_exact(&mut buffer).unwrap();
        if !process_keypress(buffer[0], &keybinds).unwrap() {
            stdout.write_all(&buffer).unwrap();
            stdout.flush().unwrap();
        }
    }
}

fn process_keypress(
    key: u8,
    keybinds: &Vec<KeyBind>,
) -> Result<bool, actions::error::Error> {
    for keybind in keybinds {
        if keybind.keys.iter().any(|k| k.key == key.to_string()) {
            let action = &keybind.on_activate;
            action()?;
            return Ok(true);
        }
    }
    Ok(false)
}
pub fn build(configuration: Settings, args: &mut Args) {
    let filename = args.nth(1);

    if let Some(file) = filename {
        let mut stdout = stdout().lock();
        let mut buffer = String::new();

        let path = PathBuf::from_str(&file).unwrap();
        FileAccess::read_from_file_if_exists(&path, &mut buffer).unwrap();

        let mut open_file = OpenFile {
            path,
            buffer: &mut buffer,
        };

        run(&mut open_file, configuration.keybinds, &mut stdout).unwrap();
    } else {
        eprintln!("No filename provided.");
        return;
    }
}
