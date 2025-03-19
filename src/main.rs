use ryvex::terminal_guard::TerminalGuard;

fn main() {
	let _guard = TerminalGuard::spawn();
	println!("Hello, world!");
	println!("Its a beautiful day!");
}
