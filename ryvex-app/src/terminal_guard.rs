use core::{
	marker::PhantomData,
	ptr,
	sync::atomic::{
		AtomicPtr,
		Ordering,
	},
};

use ryvex_target::{
	std::Result,
	target::term::{
		ConsoleSettings,
		Handle,
	},
	term::console::Console,
};

pub static TERMINAL_GUARD: AtomicPtr<TerminalGuard> =
	AtomicPtr::new(ptr::null_mut());

pub struct TerminalGuard<'a> {
	handle:       Handle,
	orig_console: ConsoleSettings,

	_phantom: PhantomData<&'a ()>,
}

impl<'a> TerminalGuard<'a> {
	pub fn spawn() -> Result<Self> {
		let (mut console, handle) = ConsoleSettings::init()?;
		let orig_console = console.raw(&handle)?;

		Ok(TerminalGuard {
			handle,
			orig_console,
			_phantom: core::marker::PhantomData,
		})
	}

	pub fn restore(&self) -> Result<()> {
		ConsoleSettings::restore(&self.handle, self.orig_console)
	}
}

impl<'a> Drop for TerminalGuard<'a> {
	fn drop(&mut self) {
		let _ = self.restore();

		TERMINAL_GUARD.store(core::ptr::null_mut(), Ordering::SeqCst);
	}
}

unsafe impl<'a> Sync for TerminalGuard<'a> {}
unsafe impl<'a> Send for TerminalGuard<'a> {}
