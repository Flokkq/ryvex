use std::io::{stdin, stdout, Read, Write};

use crate::error::{self, Error};

pub fn start() -> Result<(), error::Error> {
    let mut buf = [0; 1];
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    loop {
        stdin.read_exact(&mut buf).map_err(|err| Error::Io(err))?;

        if buf[0] == b'\x11' {
            return Ok(());
        }

        stdout.write_all(&buf)?;
        stdout.flush()?;
    }
}
