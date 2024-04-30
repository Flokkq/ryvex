use ryvex::configuration::get_configuration;
use ryvex::simulation::simulation::start;
use ryvex::startup::build;
use ryvex::terminal_guard::TerminalGuard;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let _guard = TerminalGuard::spawn().unwrap();

    if args.get(1).unwrap().to_string() == "-s" {
        let _ = start();
        drop(_guard);
        std::process::exit(0);
    }

    let configuration = get_configuration();

    let _ = build(configuration, &mut std::env::args());
}
