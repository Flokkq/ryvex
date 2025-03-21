use ryvex_app::{
	args::{
		print_help,
		Args,
	},
	error::Result,
	terminal_guard::TerminalGuard,
};
use std;

fn main() -> Result<()> {
	let exit_code = app_main()?;
	std::process::exit(exit_code)
}

fn app_main() -> Result<i32> {
	let args = Args::parse_args()?;
	if args.help_flag {
		print_help();
		return Ok(0);
	}

	let _guard = TerminalGuard::spawn()?;

	drop(_guard);

	Ok(0)
}
