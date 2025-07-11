use std::str::FromStr;

use ryvex_target::{
	r#impl::{
		TargetEnvironment,
		TargetPath,
	},
	std::env::Environment,
};

use crate::error::{
	Result,
	RyvexError,
};

#[derive(Default)]
pub struct Args {
	pub verbosity: usize,
	pub file:      Option<TargetPath>,
	pub help_flag: bool,
}

impl Args {
	pub fn parse_args(env: &TargetEnvironment) -> Result<Args> {
		let mut args = Args::default();
		let mut argv = env.args().into_iter().peekable();

		argv.next();
		for arg in argv.by_ref() {
			match arg.as_str() {
				"--" => break, // stop parsing args
				"--help" => args.help_flag = true,
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
				_ => {
					args.file =
						Some(TargetPath::from_str(arg.as_str()).unwrap());
					break;
				}
			}
		}

		if args.file.is_none() {
			args.file = argv
				.next()
				.map(|f| TargetPath::from_str(f.as_str()).unwrap())
		};

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
