use std::time::Duration;

use crate::{
	error::Result,
	event::{
		source::EventSource,
		Event,
	},
	key::AsciiKeyCode,
	sys::windows::ffi::{
		_getch,
		_kbhit,
	},
};

pub(crate) struct WindowsEventSource;

impl WindowsEventSource {
	pub fn new() -> Result<Self> {
		Ok(Self)
	}
}

impl EventSource for WindowsEventSource {
    fn try_read(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Option<Event>> {
        use std::time::{Duration, Instant};

        if timeout.is_none() {
            return Ok(Some(read_key_blocking()?));
        }

        let deadline = Instant::now() + timeout.unwrap();
        loop {
            if unsafe { _kbhit() } != 0 {
                return Ok(Some(read_key_blocking()?));
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }
            std::thread::sleep(Duration::from_millis(1));
        }

        fn read_key_blocking() -> Result<Event> {
            loop {
                let ch = unsafe { _getch() };
                if ch == 0 || ch == 0xE0 {
                    let _ = unsafe { _getch() };
                    continue;
                }
                return Ok(Event::Key(AsciiKeyCode::from_ascii(ch as u8)));
            }
        }
    }
}
