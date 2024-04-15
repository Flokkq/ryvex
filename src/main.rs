use std::io::{Read, Write};

use ryvex::terminal_guard::TerminalGuard;

use std::fs::{self, File};
use std::io::{stdin, stdout};

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1);

    if let Some(file) = filename {
        run_editor(&file).unwrap();
    } else {
        println!("No filename provided.");
    }
}

fn run_editor(filename: &str) -> Result<(), std::io::Error> {
    let _guard = TerminalGuard::spawn()?;
    let mut stdout = stdout().lock();
    let mut editor_buffer = String::new();

    if fs::metadata(filename).is_ok() {
        let mut file = File::open(filename)?;
        file.read_to_string(&mut editor_buffer)?;
        stdout.write_all(editor_buffer.as_bytes())?;
        stdout.flush()?;
    }

    loop {
        // create a buffer of size 1 initialized as 0
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer)?;

        match buffer[0] {
            // CTRL + Q
            b'\x11' => break,

            // CTRL + S or W
            b'\x13' | b'\x17' => {
                let mut file = File::create(filename)?;
                file.write_all(editor_buffer.as_bytes())?;
            }

            // BACKSPACE or DELETE
            b'\x08' | b'\x7f' => {
                if !editor_buffer.is_empty() {
                    editor_buffer.pop();
                    stdout.write_all(b"\x08 \x08")?;
                }
            }

            // ENTER
            b'\n' | b'\r' => {
                editor_buffer.push('\r');
                editor_buffer.push('\n');
                stdout.write_all(b"\r\n")?;
            }
            _ => {
                if buffer[0].is_ascii_alphanumeric()
                    || buffer[0].is_ascii_punctuation()
                    || buffer[0] == b' '
                {
                    editor_buffer.push(buffer[0] as char);
                    stdout.write_all(&buffer)?;
                }
            }
        }
        stdout.flush()?;
    }
    Ok(())
}
