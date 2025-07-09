use log::error;
use ryvex_app::{
	args::{
		print_help,
		Args,
	},
	error::Result,
	logger,
	startup::Application,
	terminal_guard::TerminalGuard,
};
use ryvex_target::term::event::SyncEventStream;
use std::{
	self,
	env::{
		self,
	},
};

fn main() -> ! {
	let guard = Box::leak(Box::new(TerminalGuard::spawn().unwrap()));
	setup_panic_handler(guard);

	let exit_code: i32 = match app_main() {
		Ok(code) => code,
		Err(e) => {
			error!("Error while running app: {}", e);
			1
		}
	};

	let _ = guard.restore();
	std::process::exit(exit_code)
}

fn app_main() -> Result<i32> {
	let args = Args::parse_args()?;

	if args.help_flag {
		print_help();
		return Ok(0);
	}

	setup_logging(args.verbosity)?;
	let mut app = Application::build(args)?;

	let mut event_stream = SyncEventStream::new()?;
	let exit_code = app.run_until_stopped(&mut event_stream)?;

	Ok(exit_code)
}

fn setup_panic_handler(guard: &'static TerminalGuard<'static>) {
	let original_hook = std::panic::take_hook();

	std::panic::set_hook(Box::new(move |info| {
		let _ = guard.restore();

		original_hook(info);
	}));
}

fn setup_logging(verbosity: usize) -> Result<()> {
	match verbosity {
		0 => env::set_var("RUST_LOG", "warn"),
		1 => env::set_var("RUST_LOG", "info"),
		2 => env::set_var("RUST_LOG", "debug"),
		_3_or_more => env::set_var("RUST_LOG", "trace"),
	}
	logger::init()
}
