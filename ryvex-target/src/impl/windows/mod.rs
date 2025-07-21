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
	const LOG_DIR: &'static str = "ryvex";
	const LOG_DIR_BASE: &'static str = "C:\\var\\log";
}

pub use WindowsScheme as TargetPathScheme;
