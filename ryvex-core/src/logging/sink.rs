use ryvex_target::std::{
	error::IoError,
	write::Write,
};

use super::{
	record::OwnedRecord,
	spinlock::SpinLock,
};

#[derive(Clone, Copy)]
pub enum SinkState {
	Uninitialized,
	Ready,
}

pub struct Sink {
	state:           SinkState,
	buffer:          Vec<u8>,
	writer:          Option<Box<dyn Write + Send>>,
	auto_flush_byes: usize,
}

impl Default for Sink {
	fn default() -> Self {
		Self::new()
	}
}

impl Sink {
	pub const fn new() -> Self {
		Self {
			state:           SinkState::Uninitialized,
			buffer:          Vec::new(),
			writer:          None,
			auto_flush_byes: 4096,
		}
	}

	pub fn set_writer<W: Write + Send + 'static>(&mut self, writer: W) {
		self.writer = Some(Box::new(writer));
		self.state = SinkState::Ready;

		let _ = self.flush();
	}

	pub fn write_record(&mut self, r: &OwnedRecord) {
		let line = r.to_string();

		self.buffer.extend_from_slice(line.as_bytes());

		if self.buffer.len() >= self.auto_flush_byes {
			let _ = self.flush();
		}
	}

	pub fn flush(&mut self) -> Result<(), IoError> {
		if let Some(w) = self.writer.as_mut() {
			if !self.buffer.is_empty() {
				w.write_all(&self.buffer)?;
				w.flush()?;

				self.buffer.clear();
			}
		}

		Ok(())
	}
}

pub struct LogSinkControl<'a> {
	pub(crate) sink: &'a SpinLock<Sink>,
}

impl<'a> LogSinkControl<'a> {
	pub fn flush(&self) -> Result<(), IoError> {
		self.sink.lock().flush()
	}

	pub fn state(&self) -> SinkState {
		self.sink.lock().state
	}

	pub fn set_auto_flush_bytes(&self, n: usize) {
		self.sink.lock().auto_flush_byes = n;
	}
}
