use ryvex_app::{
	terminal_guard::TerminalGuard,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let _guard = TerminalGuard::spawn()?;

	drop(_guard);
	std::process::exit(0)
}
