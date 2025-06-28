use super::EventSource;

use crate::error::Result;

pub(crate) struct WindowsEventSource;

impl WindowsEventSource {
	pub fn new() -> Result<Self> {
		Ok(Self {})
	}
}

impl EventSource for WindowsEventSource {
	fn try_read(
		&mut self,
		_timeout: Option<std::time::Duration>,
	) -> crate::error::Result<Option<crate::event::Event>> {
		todo!()
	}
}
