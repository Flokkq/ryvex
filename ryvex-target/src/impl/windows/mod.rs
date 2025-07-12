mod ffi;

pub mod term;

#[derive(Debug, Clone, Copy, Default)]
pub struct WindowsScheme;

impl crate::std::path::PathScheme for WindowsScheme {
	const MAIN_SEPARATOR: char = '\\';
	const ALT_SEPARATOR: &'static [char] = &['/'];
	const EXTENSION_SEPARATOR: char = '.';
	const CURRENT_DIR: &'static str = ".";
	const PARENT_DIR: &'static str = "..";
}

pub use WindowsScheme as TargetPathScheme;
