use std::{
    env::Args,
    io::Result,
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
    /*     keybinds: Vec<KeyBind<fn() -> Result<(), actions::error::Error>>>, */
    stdout: &mut StdoutLock,
) -> Result<()> {
    stdout.write_all(file.buffer.as_bytes())?;
    loop {
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer)?;

        match buffer[0] {
            b'\x11' => break,
            b'\x13' | b'\x17' => {
                FileAccess::write_to_file(&file.path, file.buffer)?;
            }
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
        stdout.flush()?;
    }
    Ok(())
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

        run(
            &mut open_file,
            /* configuration.keybinds ,*/ &mut stdout,
        )
        .unwrap();
    } else {
        eprintln!("No filename provided.");
        return;
    }
}
