use std::{
    io::{StdoutLock, Write},
    path::PathBuf,
};

use crate::{core::error::Error, file_access::FileAccess};

use super::buffer::Buffer;

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

    pub fn redraw(&self, stdout: &mut StdoutLock) -> Result<(), Error> {
        stdout.write_all("\x1B[2J".as_bytes())?;

        stdout.write_all("\x1B[H".as_bytes())?;

        self.buffer.display(stdout)?;
        stdout.flush()?;
        Ok(())
    }
}
