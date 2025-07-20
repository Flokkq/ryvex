use core::sync::atomic::{
	AtomicU8,
	Ordering,
};

use chain::{
	ErrorChain,
	ErrorFrame,
	TraceData,
	MAX_ERROR_FRAMES,
};
use record::{
	LogLevel,
	OwnedRecord,
	RecordSnapshot,
};
use ring::Ring;
use ryvex_target::std::{
	error::Error,
	write::Write,
};
use sink::{
	LogSinkControl,
	Sink,
};
use spinlock::SpinLock;

pub mod chain;
pub mod macros;
pub mod record;
pub mod ring;
pub mod sink;
pub mod spinlock;

pub const LOG_CAPACITY: usize = 512;

pub struct Logger<const N: usize> {
	inner:      SpinLock<Inner<N>>,
	sink:       SpinLock<Sink>,
	level_mask: AtomicU8,
}

struct Inner<const N: usize> {
	ring:           Ring<N>,
	last_info_warn: u64,
	last_error_seq: u64,
}

impl<const N: usize> Inner<N> {
	pub const fn new() -> Self {
		Self {
			ring:           Ring::new(),
			last_info_warn: 0,
			last_error_seq: 0,
		}
	}
}

pub static LOGGER: Logger<LOG_CAPACITY> = Logger::new();

impl<const N: usize> Default for Logger<N> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const N: usize> Logger<N> {
	pub const fn new() -> Self {
		Self {
			inner:      SpinLock::new(Inner::new()),
			sink:       SpinLock::new(Sink::new()),
			level_mask: AtomicU8::new(
				LogLevel::Error.bit() |
					LogLevel::Warn.bit() |
					LogLevel::Info.bit() |
					LogLevel::Debug.bit() |
					LogLevel::Trace.bit(),
			),
		}
	}

	pub fn init_with_target_out<W: Write + Send + 'static>(&self, writer: W) {
		self.sink.lock().set_writer(writer);
	}

	pub fn sink_control(&self) -> LogSinkControl {
		LogSinkControl { sink: &self.sink }
	}

	pub fn set_enabled(&self, level: LogLevel, enabled: bool) {
		let mut mask = self.level_mask.load(Ordering::Relaxed);

		if enabled {
			mask |= level.bit();
		} else {
			mask &= !level.bit();
		}

		self.level_mask.store(mask, Ordering::Relaxed);
	}

	pub fn enabled(&self, level: LogLevel) -> bool {
		(self.level_mask.load(Ordering::Relaxed) & level.bit()) != 0
	}

	pub fn toggle_expanded(&self, seq: u64) {
		let inner = self.inner.lock();

		if let Some(r) = inner.ring.get(seq) {
			let cur = r.is_expanded();
			r.set_expanded(!cur);
		}
	}

	pub fn latest_info_warn(&self) -> Option<RecordSnapshot> {
		let inner = self.inner.lock();

		if inner.ring.write_index == 0 {
			return None;
		}

		let seq = inner.last_info_warn;
		drop(inner);

		self.snapshot(seq)
	}

	pub fn recent_errors(
		&self,
		limit: usize,
	) -> alloc::vec::Vec<RecordSnapshot> {
		let inner = self.inner.lock();

		let mut out = alloc::vec::Vec::new();
		for rec in inner.ring.iter_recent().rev() {
			if rec.level == LogLevel::Error {
				out.push(rec.snapshot());
				if out.len() == limit {
					break;
				}
			}
		}

		out.reverse();
		out
	}

	pub fn latest_error_seq(&self) -> u64 {
		self.inner.lock().last_error_seq
	}

	pub fn flush(&self) {
		self.sink.lock().flush().ok();
	}

	pub fn record(
		&self,
		level: LogLevel,
		module: &'static str,
		file: &'static str,
		line: u32,
		msg: String,
		trace: TraceData,
	) {
		let mut inner = self.inner.lock();
		let seq = inner.ring.write_index;
		let rec = record::OwnedRecord::new(
			seq, level, module, file, line, msg, trace,
		);
		let seq_written = inner.ring.push(rec);

		match level {
			LogLevel::Info | LogLevel::Warn => {
				inner.last_info_warn = seq_written
			}
			LogLevel::Error => inner.last_error_seq = seq_written,
			_ => {}
		}
		drop(inner);

		self.with_record(seq_written, |r| {
			self.sink.lock().write_record(r);
		});
	}

	pub fn with_record(&self, seq: u64, f: impl FnOnce(&OwnedRecord)) {
		let inner = self.inner.lock();
		if let Some(r) = inner.ring.get(seq) {
			f(r);
		}
	}

	pub fn snapshot(&self, seq: u64) -> Option<RecordSnapshot> {
		let inner = self.inner.lock();
		inner.ring.get(seq).map(|r| r.snapshot())
	}
}

pub fn log_msg_internal(
	level: LogLevel,
	args: core::fmt::Arguments,
	module: &'static str,
	file: &'static str,
	line: u32,
) {
	let msg = format!("{}", args);

	LOGGER.record(level, module, file, line, msg, TraceData::None);
}

pub fn log_error_chain(
	err: &(dyn Error + 'static),
	ctx: Option<core::fmt::Arguments>,
	module: &'static str,
	file: &'static str,
	line: u32,
) {
	let mut chain = ErrorChain::capture(err);

	if let Some(ca) = ctx {
		let ctx_str = format!("{}", ca);

		if chain.frames.first().map(|f| f.msg.as_str()) !=
			Some(ctx_str.as_str()) &&
			chain.frames.len() < MAX_ERROR_FRAMES
		{
			chain.frames.insert(0, ErrorFrame { msg: ctx_str });
		}
	}

	let top_msg = chain
		.frames
		.first()
		.map(|f| f.msg.clone())
		.unwrap_or_else(|| String::from("<error>"));

	LOGGER.record(
		LogLevel::Error,
		module,
		file,
		line,
		top_msg,
		TraceData::ErrorChain(chain),
	);
}
