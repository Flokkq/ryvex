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
}

pub use UnixScheme as TargetPathScheme;
