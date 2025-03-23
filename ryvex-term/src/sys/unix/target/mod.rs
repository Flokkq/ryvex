//! OS-specific definitions.
#![allow(non_camel_case_types)]

#[cfg(target_os = "linux")]
pub use self::linux as os;
#[cfg(target_os = "macos")]
pub use self::macos as os;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;

use std::ffi::c_int;

pub const STDIN_FILENO: c_int = 0;
pub type ssize_t = isize;
