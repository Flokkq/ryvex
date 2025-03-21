use ryvex_app::{
	args::{
		print_help,
		Args,
	},
	error::Result,
	logger,
	terminal_guard::TerminalGuard,
};
use std::{
	self,
	env::{
		self,
	},
};

fn main() -> Result<()> {
	let exit_code = app_main()?;
	std::process::exit(exit_code)
}

fn app_main() -> Result<i32> {
	let args = Args::parse_args()?;
	if args.verbosity == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbosity > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;

	if args.help_flag {
		print_help();
		return Ok(0);
	}

	let _guard = TerminalGuard::spawn()?;

	drop(_guard);

	Ok(0)
}
