use ryvex::configuration::get_configuration;
use ryvex::startup::build;
use ryvex::terminal_guard::TerminalGuard;

fn main() {
    let _guard = TerminalGuard::spawn().unwrap();

    let configuration =
        get_configuration().expect("Failed to build configuration");

    build(configuration, &mut std::env::args());
}
