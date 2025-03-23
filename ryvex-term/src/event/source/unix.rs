use super::EventSource;

pub(crate) struct UnixEventSource;

impl UnixEventSource {
	pub fn new() -> Self {
		Self {}
	}
}

impl EventSource for UnixEventSource {
	fn try_read(
		&mut self,
		_timeout: Option<std::time::Duration>,
	) -> crate::error::Result<Option<crate::event::Event>> {
		todo!();
	}
}
