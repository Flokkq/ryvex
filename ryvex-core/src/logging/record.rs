use core::{
	fmt::Display,
	sync::atomic::{
		AtomicBool,
		Ordering,
	},
};

use super::chain::{
	ErrorChain,
	TraceData,
};

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum LogLevel {
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

impl LogLevel {
	pub const fn bit(self) -> u8 {
		1 << (self as u8)
	}
}

impl Display for LogLevel {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let s = match self {
			LogLevel::Error => "ERROR",
			LogLevel::Warn => "WARN",
			LogLevel::Info => "INFO",
			LogLevel::Debug => "DEBUG",
			LogLevel::Trace => "TRACE",
		};

		write!(f, "{}", s)
	}
}

pub struct OwnedRecord {
	pub seq:         u64,
	pub level:       LogLevel,
	pub module_path: &'static str,
	pub file:        &'static str,
	pub line:        u32,
	pub msg:         String,
	pub trace:       TraceData,
	expanded:        AtomicBool,
}

#[derive(Clone)]
pub struct RecordSnapshot {
	pub seq:         u64,
	pub level:       LogLevel,
	pub module_path: &'static str,
	pub file:        &'static str,
	pub line:        u32,
	pub msg:         String,
	pub chain:       Option<Vec<String>>,
	pub truncated:   bool,
	pub expanded:    bool,
}

impl OwnedRecord {
	pub fn new(
		seq: u64,
		level: LogLevel,
		module_path: &'static str,
		file: &'static str,
		line: u32,
		msg: String,
		trace: TraceData,
	) -> Self {
		Self {
			seq,
			level,
			module_path,
			file,
			line,
			msg,
			trace,
			expanded: AtomicBool::new(false),
		}
	}

	pub fn snapshot(&self) -> RecordSnapshot {
		let (chain, truncated) = match &self.trace {
			TraceData::ErrorChain(ec) => (
				Some(ec.frames.iter().map(|f| f.msg.clone()).collect()),
				ec.truncated,
			),
			_ => (None, false),
		};

		RecordSnapshot {
			seq: self.seq,
			level: self.level,
			module_path: self.module_path,
			file: self.file,
			line: self.line,
			msg: self.msg.clone(),
			chain,
			truncated,
			expanded: self.is_expanded(),
		}
	}

	pub fn set_expanded(&self, v: bool) {
		self.expanded.store(v, Ordering::Relaxed);
	}

	pub fn is_expanded(&self) -> bool {
		self.expanded.load(Ordering::Relaxed)
	}
}

impl Display for OwnedRecord {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut line = format!(
			"[{}] {} {} {}:{} - {}\n",
			self.seq,
			self.level,
			self.module_path,
			self.file,
			self.line,
			self.msg
		);

		if let TraceData::ErrorChain(ErrorChain { truncated, frames }) =
			&self.trace
		{
			for (i, f) in frames.iter().enumerate() {
				line.push_str(&format!("\tchain[{i}]: {}\n", f.msg));
			}

			if *truncated {
				line.push_str("\t...(truncated)\n");
			}
		}

		write!(f, "{}", line)
	}
}
