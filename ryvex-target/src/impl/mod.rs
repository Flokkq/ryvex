#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod unix;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use unix as target;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows as target;
