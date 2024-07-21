use std::{
    io::{StdoutLock, Write},
    path::PathBuf,
};

use crate::{core::error::Error, file_access::FileAccess};

use super::buffer::{Buffer, Direction};

pub struct OpenFile {
    pub path: PathBuf,
    pub buffer: Buffer,
}

impl OpenFile {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let mut buffer = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut buffer)?;

        Ok(OpenFile {
            path,
            buffer: Buffer::new(buffer),
        })
    }

    pub fn is_empty(&mut self) -> bool {
        self.buffer.is_empty()
    }

    pub fn insert(&mut self, ch: char) {
        self.buffer.insert(ch);
    }

    pub fn insert_newline(&mut self) {
        self.buffer.insert_newline();
    }

    pub fn delete(&mut self) {
        self.buffer.delete();
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        self.buffer.move_cursor(direction);
    }

    pub fn redraw(&self, stdout: &mut StdoutLock) -> Result<(), Error> {
        stdout.write_all("\x1B[2J".as_bytes())?;

        stdout.write_all("\x1B[H".as_bytes())?;

        self.buffer.display(stdout)?;
        stdout.flush()?;
        Ok(())
    }
}
