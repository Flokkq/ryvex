use core::time::Duration;

use crate::{
	key::AsciiKeyCode,
	std::Result,
	target::term::TargetEventSource,
};

#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum Event {
	Key(AsciiKeyCode),
	Resize(u16, u16),
}

pub trait EventSource: Sync + Send {
	/// Block until an event is available (or until timeout, if provided).
	fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<Event>>;
}

pub struct SyncEventStream {
	inner: Box<dyn EventSource>,
}

impl SyncEventStream {
	pub fn new() -> Result<Self> {
		let source = TargetEventSource::new()?;

		Ok(Self {
			inner: Box::new(source),
		})
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
