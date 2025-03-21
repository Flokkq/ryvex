use ryvex_app::{
	error::Result,
	terminal_guard::TerminalGuard,
};
use std;

fn main() -> Result<()> {
	let exit_code = app_main()?;
	std::process::exit(exit_code)
}

fn app_main() -> Result<i32> {
	let _guard = TerminalGuard::spawn()?;

	drop(_guard);

	Ok(0)
}
