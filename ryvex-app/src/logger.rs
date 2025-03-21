use env_logger::{
	fmt::{
		Color,
		Style,
		StyledValue,
	},
	Builder,
	Target,
};
use log::Level;
use std::io::Write;
use std::sync::atomic::{
	AtomicUsize,
	Ordering,
};
use std::{
	env,
	fmt,
};

use crate::error::{
	Result,
	RyvexError,
};

/// Environment variable to use for the logger.
const LOGGER_ENV: &str = "RUST_LOG";

/// Global variable for storing the maximum width of the modules.
static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

/// Wrapper for the padded values.
struct Padded<T> {
	value: T,
	width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{: <width$}", self.value, width = self.width)
	}
}

/// Returns the max width of the target.
fn max_target_width(target: &str) -> usize {
	let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
	if max_width < target.len() {
		MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
		target.len()
	} else {
		max_width
	}
}

/// Adds colors to the given level and returns it.
fn colored_level(
	style: &mut Style,
	level: Level,
) -> StyledValue<'_, &'static str> {
	match level {
		Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
		Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
		Level::Info => style.set_color(Color::Green).value("INFO "),
		Level::Warn => style.set_color(Color::Yellow).value("WARN "),
		Level::Error => style.set_color(Color::Red).value("ERROR"),
	}
}

/// Initializes the global logger.
#[allow(unreachable_code, clippy::needless_return)]
pub fn init() -> Result<()> {
	let mut builder = Builder::new();
	builder.target(Target::Stderr);

	builder.format(move |f, record| {
		let target = record.target();
		let max_width = max_target_width(target);

		let mut style = f.style();
		let level = colored_level(&mut style, record.level());

		let mut style = f.style();
		let target = style.set_bold(true).value(Padded {
			value: target,
			width: max_width,
		});

		writeln!(f, " {} {} > {}", level, target, record.args())
	});

	if let Ok(var) = env::var(LOGGER_ENV) {
		builder.parse_filters(&var);
	}

	builder
		.try_init()
		.map_err(|e| RyvexError::LoggerError(e.to_string()))
}
