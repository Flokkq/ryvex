use super::ffi;
use crate::{
	key::AsciiKeyCode,
	term::event::{
		Event,
		EventSource,
	},
};
use std::{
	io,
	time::{
		Duration,
		Instant,
	},
};

pub struct WindowsEventSource;

impl WindowsEventSource {
	pub fn new() -> io::Result<Self> {
		Ok(Self)
	}
}

impl EventSource for WindowsEventSource {
	fn try_read(
		&mut self,
		timeout: Option<Duration>,
	) -> io::Result<Option<Event>> {
		if timeout.is_none() {
			return Ok(Some(read_key_blocking()?));
		}

		let deadline = Instant::now() + timeout.unwrap();
		loop {
			if ffi::kbhit() {
				return Ok(Some(read_key_blocking()?));
			}
			if Instant::now() >= deadline {
				return Ok(None);
			}
			std::thread::sleep(Duration::from_millis(1));
		}
	}
}

fn read_key_blocking() -> io::Result<Event> {
	loop {
		let ch = ffi::getch();
		if ch == 0 || ch == 0xE0 {
			let _ = ffi::getch();
			continue;
		}
		return Ok(Event::Key(AsciiKeyCode::from_ascii(ch as u8)));
	}
}
