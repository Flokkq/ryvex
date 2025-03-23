use std::time::Duration;

use crate::error::Result;

use super::Event;

#[cfg(not(target_os = "windows"))]
pub(crate) mod unix;
#[cfg(target_os = "windows")]
pub(crate) mod windows;

pub trait EventSource: Sync + Send {
	/// Block until an event is available (or until timeout, if provided).
	fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<Event>>;
}
