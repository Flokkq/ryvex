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
use ryvex_term::event::stream::SyncEventStream;
use std::{
	self,
	env::{
		self,
	},
};

fn main() -> Result<()> {
	let exit_code = app_main().map_err(|e| {
		error!("Error while running app: {}", e);
		e
	})?;
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

	let _guard = TerminalGuard::spawn()?;

	let mut event_stream = SyncEventStream::new()?;
	let exit_code = app.run_until_stopped(&mut event_stream)?;

	Ok(exit_code)
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
