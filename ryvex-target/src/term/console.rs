use std::io;

/// Abstraction over a resource handle (e.g. a terminal file descriptor)
///
/// # Type Parameters
/// - `T`: The underlying handle type (e.g. [`RawFd`] on Unix)
/// - `S`: Settings for the underlying handle (e.g. read/write)
pub trait Handle<T, S>
where
	Self: Sized,
{
	// Acquire a new handle instance.
	fn acquire(mode: S) -> io::Result<Self>;

	fn inner(&self) -> &T;

	fn inner_mut(&mut self) -> &mut T;
}

/// Trait for managing console settings and modes
///
/// # Type Parameters
/// - `T`: the handle type used by this console implementation
pub trait Console<T, S>
where
	Self: Sized,
{
	type Handle: Handle<T, S>;

	/// Read the current console settings form the handle and return a new
	/// instance capturing the saved state;
	fn init() -> io::Result<(Self, Self::Handle)>;

	/// Switch the console into raw (unprocessed) mode, return the previous
	/// configuration so it can later be restored.
	fn raw(&mut self, fd: &Self::Handle) -> io::Result<Self>;

	/// Restore the console to a previously saved configuration.
	///
	/// # Parameters
	/// - `orig`: the state returned by a prior call to `raw`.
	fn restore(fd: &Self::Handle, orig: Self) -> io::Result<()>;
}
