#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
extern crate alloc;

use core::sync::atomic::Ordering;
#[cfg(not(feature = "std"))]
use core::{
	error::Error,
	panic::PanicInfo,
	ptr,
};
use ryvex_app::terminal_guard::TERMINAL_GUARD;

use alloc::boxed::Box;

use ryvex_app::{
	args::{
		self,
		Args,
	},
	error::Result,
	startup::Application,
	terminal_guard::TerminalGuard,
};
use ryvex_target::{
	target::TargetContext,
	term::event::SyncEventStream,
};

fn main() -> ! {
	let exit_code: i32 = app_main().unwrap_or(1);

	ryvex_target::target::exit(exit_code)
}

fn app_main() -> Result<i32> {
	let cx = TargetContext::default();
	let args = Args::parse_args(&cx.env)?;

	if args.help_flag {
		args::print_help(&cx.env);
		return Ok(0);
	}

	let guard = Box::new(TerminalGuard::spawn()?);
	TERMINAL_GUARD
		.store(&*guard as *const _ as *mut TerminalGuard, Ordering::SeqCst);

	#[cfg(feature = "std")]
	setup_panic_handler();

	// setup_logging(&cx.env, args.verbosity)?;
	let mut app = Application::build(cx, args)?;

	let mut event_stream = SyncEventStream::new()?;
	let exit_code = app.run_until_stopped(&mut event_stream)?;

	let _ = guard.restore();
	Ok(exit_code)
}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	let ptr = TERMINAL_GUARD.load(Ordering::SeqCst);
	if !ptr.is_null() {
		unsafe { (&*ptr).restore() };
	}

	core::intrinsics::abort()
}

#[cfg(feature = "std")]
fn setup_panic_handler() {
	let original_hook = std::panic::take_hook();

	std::panic::set_hook(Box::new(move |info| {
		let ptr = TERMINAL_GUARD.load(Ordering::SeqCst);

		if !ptr.is_null() {
			let _ = unsafe { (*ptr).restore() };
		}

		original_hook(info);
	}));
}

// fn setup_logging(env: &TargetEnvironment, verbosity: usize) -> Result<()> {
// 	match verbosity {
// 		0 => env.set_var("RUST_LOG", "warn"),
// 		1 => env.set_var("RUST_LOG", "info"),
// 		2 => env.set_var("RUST_LOG", "debug"),
// 		_3_or_more => env.set_var("RUST_LOG", "trace"),
// 	}
// 	logger::init()
// }
