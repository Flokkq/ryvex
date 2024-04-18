use ryvex::configuration::get_configuration;
use ryvex::startup::build;
use ryvex::terminal_guard::TerminalGuard;

fn main() {
    let guard = TerminalGuard::spawn().unwrap();

    let configuration = get_configuration();

    match build(configuration, &mut std::env::args()) {
        Ok(()) => {}
        Err(_) => drop(guard),
    }
}
