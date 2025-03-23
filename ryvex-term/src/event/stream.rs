use super::{
	source::EventSource,
	Event,
};

#[cfg(unix)]
use super::source::unix::UnixEventSource as TargetEventSource;
#[cfg(windows)]
use super::source::unix::WindowsEventSource as TargetEventSource;

use crate::error::Result;

pub struct SyncEventStream {
	inner: Box<dyn EventSource>,
}

impl SyncEventStream {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for SyncEventStream {
	fn default() -> Self {
		let source = TargetEventSource::new();

		Self {
			inner: Box::new(source),
		}
	}
}

impl Iterator for SyncEventStream {
	type Item = Result<Event>;

	/// Block indefinitely until an event is available.
	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.try_read(None) {
			Ok(Some(event)) => Some(Ok(event)),
			Ok(None) => {
				// If we get None, simply try again.
				self.next()
			}
			Err(e) => Some(Err(e)),
		}
	}
}
