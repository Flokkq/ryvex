use super::ffi;
use crate::{
	key::AsciiKeyCode,
	std::Result,
	term::event::{
		Event,
		EventSource,
	},
};
use std::time::{
	Duration,
	Instant,
};

pub struct WindowsEventSource;

impl WindowsEventSource {
	pub fn new() -> Result<Self> {
		Ok(Self)
	}
}

impl EventSource for WindowsEventSource {
	fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<Event>> {
		if timeout.is_none() {
			return Ok(Some(read_key_blocking()));
		}

		let deadline = Instant::now() + timeout.unwrap();
		loop {
			if ffi::kbhit() {
				return Ok(Some(read_key_blocking()));
			}
			if Instant::now() >= deadline {
				return Ok(None);
			}
			std::thread::sleep(Duration::from_millis(1));
		}
	}
}

fn read_key_blocking() -> Event {
	loop {
		let ch = ffi::getch();
		if ch == 0 || ch == 0xE0 {
			let _ = ffi::getch();
			continue;
		}
		return Event::Key(AsciiKeyCode::from_ascii(ch as u8));
	}
}
