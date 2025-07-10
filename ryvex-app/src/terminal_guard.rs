use std::{
	io,
	marker::PhantomData,
};

use ryvex_target::{
	target::term::{
		ConsoleSettings,
		Handle,
	},
	term::console::Console,
};

pub struct TerminalGuard<'a> {
	handle:       Handle,
	orig_console: ConsoleSettings,

	_phantom: PhantomData<&'a ()>,
}

impl<'a> TerminalGuard<'a> {
	pub fn spawn() -> io::Result<Self> {
		let (mut console, handle) = ConsoleSettings::init()?;
		let orig_console = console.raw(&handle)?;

		Ok(TerminalGuard {
			handle,
			orig_console,
			_phantom: std::marker::PhantomData,
		})
	}

	pub fn restore(&self) -> io::Result<()> {
		ConsoleSettings::restore(&self.handle, self.orig_console)
	}
}

impl<'a> Drop for TerminalGuard<'a> {
	fn drop(&mut self) {
		let _ = self.restore();
	}
}

unsafe impl<'a> Sync for TerminalGuard<'a> {}
unsafe impl<'a> Send for TerminalGuard<'a> {}
