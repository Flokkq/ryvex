#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use self::unix as target;
#[cfg(windows)]
pub use self::windows as target;
