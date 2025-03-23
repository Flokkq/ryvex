use std::time::Duration;

use crate::error::Result;

use super::Event;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

pub trait EventSource: Sync + Send {
	/// Block until an event is available (or until timeout, if provided).
	fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<Event>>;
}
