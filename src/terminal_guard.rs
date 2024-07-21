use std::{io::stdin, os::fd::AsRawFd};

use termios::*;

pub struct TerminalGuard<'a> {
    fd: i32,
    orig_termios: Termios,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> TerminalGuard<'a> {
    pub fn spawn() -> Result<TerminalGuard<'a>, std::io::Error> {
        let stdin = stdin();
        let stdin_fd = stdin.as_raw_fd();
        let orig_termios = Termios::from_fd(stdin_fd)?;

        let mut termios = orig_termios;

        // Sets flags to disable all input and output processing.
        cfmakeraw(&mut termios);
        tcsetattr(stdin_fd, TCSANOW, &termios)?;

        Ok(TerminalGuard {
            fd: stdin_fd,
            orig_termios,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<'a> Drop for TerminalGuard<'a> {
    fn drop(&mut self) {
        let _ = tcsetattr(self.fd, TCSANOW, &self.orig_termios);
    }
}
