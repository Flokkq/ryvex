mod ffi;
mod target;

pub mod term;

#[derive(Debug, Clone, Copy, Default)]
pub struct UnixScheme;

impl crate::std::path::PathScheme for UnixScheme {
	const MAIN_SEPARATOR: char = '/';
	const ALT_SEPARATOR: &'static [char] = &[];
	const EXTENSION_SEPARATOR: char = '.';
	const CURRENT_DIR: &'static str = ".";
	const PARENT_DIR: &'static str = "..";
	const LOG_DIR_BASE: &'static str = "~/.local/share";
	const LOG_DIR: &'static str = "ryvex/logs";
}

pub use UnixScheme as TargetPathScheme;
