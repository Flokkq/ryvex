use crate::error::{
	Result,
	RyvexError,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct Args {
	pub verbosity: usize,
	pub file:      Option<PathBuf>,
	pub help_flag: bool,
}

impl Args {
	pub fn parse_args() -> Result<Args> {
		let mut args = Args::default();
		let mut argv = std::env::args().peekable();

		argv.next();
		while let Some(arg) = argv.next() {
			match arg.as_str() {
				"--" => break, // stop parsing args
				"--help" => print_help(),
				arg if arg.starts_with("--") => {
					return Err(RyvexError::ArgParseError(format!(
						"unexpected long arg {}",
						arg
					)));
				}
				arg if arg.starts_with('-') => {
					let arg = arg.get(1..).unwrap().chars();
					for chr in arg {
						match chr {
							'v' => args.verbosity += 1,
							'h' => args.help_flag = true,
							_ => {
								return Err(RyvexError::ArgParseError(format!(
									"unexpected arg -{}",
									chr
								)))
							}
						}
					}
				}
				_ => {}
			}
		}
		args.file = argv.next().map(|f| PathBuf::from(f));

		Ok(args)
	}
}

pub fn print_help() {
	println!(
		r#"
{} {}
{}
{}
Usage:
    ryvex [FLAGS] [file]

ARGS: <file>            File to open, if not provided, will open a new buffer

FLAGS:
    -h, --help          Prints help information
    -v                  Increase verbosity
"#,
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		env!("CARGO_PKG_AUTHORS"),
		env!("CARGO_PKG_DESCRIPTION")
	);
}
