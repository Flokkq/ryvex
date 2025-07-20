use crate::compositor::{
	Component,
	Context,
	EventResult,
};
use alloc::format;
use ryvex_core::logging::record::RecordSnapshot;
use ryvex_core::logging::LOGGER;
use ryvex_target::{
	key::AsciiKeyCode,
	term::event::Event,
};
use ryvex_tui::buffer::Buffer;
use ryvex_ui::graphics::Rect;

#[derive(Clone, Copy, Debug)]
pub enum LoggingCommand {
	Acknowledge,
}

pub struct LoggingKeyMap;

impl LoggingKeyMap {
	pub fn new() -> Self {
		Self
	}
	pub fn feed(&self, key: AsciiKeyCode) -> Option<LoggingCommand> {
		match key {
			AsciiKeyCode::CarriageReturn => Some(LoggingCommand::Acknowledge),
			_ => None,
		}
	}
}

pub struct LoggingLine {
	active_error:        Option<RecordSnapshot>,
	last_error_seq_seen: u64,
	last_info_warn:      Option<RecordSnapshot>,
	keymap:              LoggingKeyMap,
	dirty:               bool,
}

impl Default for LoggingLine {
	fn default() -> Self {
		Self::new()
	}
}

impl LoggingLine {
	pub fn new() -> Self {
		Self {
			active_error:        None,
			last_error_seq_seen: 0,
			last_info_warn:      None,
			keymap:              LoggingKeyMap::new(),
			dirty:               true,
		}
	}

	fn poll_logger(&mut self) {
		if self.active_error.is_some() {
			return;
		}

		if let Some(latest_err) = LOGGER.recent_errors(1).into_iter().next() {
			if latest_err.seq > self.last_error_seq_seen {
				self.active_error = Some(latest_err.clone());
				self.last_error_seq_seen = latest_err.seq;
				self.dirty = true;
				return;
			}
		}

		if let Some(info_warn) = LOGGER.latest_info_warn() {
			if self
				.last_info_warn
				.as_ref()
				.map(|s| s.seq != info_warn.seq)
				.unwrap_or(true)
			{
				self.last_info_warn = Some(info_warn);
				self.dirty = true;
			}
		}
	}

	fn close_error(&mut self) {
		if self.active_error.take().is_some() {
			self.dirty = true;
		}

		if self.last_info_warn.is_none() {
			if let Some(info_warn) = LOGGER.latest_info_warn() {
				self.last_info_warn = Some(info_warn);
			}
		}
	}

	pub fn invalidate(&mut self) {
		self.dirty = true;
	}
}

struct LineView<'a> {
	snap:  &'a RecordSnapshot,
	width: u16,
}
impl<'a> core::fmt::Display for LineView<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut line = self.snap.msg.to_string();

		if line.len() > self.width as usize {
			line.truncate(self.width as usize);
		}

		write!(f, "{line}")
	}
}

struct ErrorFrameView<'a> {
	snap:   &'a RecordSnapshot,
	width:  u16,
	height: u16,
}

impl<'a> core::fmt::Display for ErrorFrameView<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let header = format!(
			"[ERROR seq={}] {} ({}:{}:{})",
			self.snap.seq,
			self.snap.msg,
			self.snap.module_path,
			self.snap.file,
			self.snap.line
		);

		let mut lines = alloc::vec::Vec::new();
		lines.push(header);

		if let Some(chain) = &self.snap.chain {
			for (i, frame) in chain.iter().enumerate() {
				lines.push(format!("{:>2}: {frame}", i));
			}

			if self.snap.truncated {
				lines.push("..(truncated)".into());
			}
		}

		if lines.len() as u16 > self.height {
			lines.truncate(self.height as usize);
		}

		for (i, l) in lines.iter().enumerate() {
			let mut seg = l.clone();
			if seg.len() > self.width as usize {
				seg.truncate(self.width as usize);
			}
			if i > 0 {
				writeln!(f)?;
			}
			write!(f, "{seg}")?;
		}

		Ok(())
	}
}

impl Component for LoggingLine {
	fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
		if self.active_error.is_some() {
			let h = core::cmp::max(1, viewport.1 / 3);
			Some((viewport.0, h))
		} else {
			Some((viewport.0, 1))
		}
	}

	fn render(&mut self, area: Rect, frame: &mut Buffer, _cx: &mut Context) {
		self.poll_logger();
		if let Some(err) = &self.active_error {
			let h = core::cmp::max(1, area.height / 3);
			let y0 = area.y + area.height - h;
			let error_rect = Rect {
				x:      area.x,
				y:      y0,
				width:  area.width,
				height: h,
			};

			let view = ErrorFrameView {
				snap:   err,
				width:  error_rect.width,
				height: error_rect.height,
			};

			let mut y = error_rect.y;
			for (i, line) in view.to_string().split('\n').enumerate() {
				if (i as u16) >= error_rect.height {
					break;
				}
				frame.set_string(error_rect.x, y, line);
				y += 1;
			}
		} else if let Some(line_snap) = &self.last_info_warn {
			let view = LineView {
				snap:  line_snap,
				width: area.width,
			};

			frame.set_string(
				area.x,
				area.y + area.height - 1,
				view.to_string(),
			);
		} else {
			frame.set_string(area.x, area.y + area.height - 1, "");
		}

		self.dirty = false;
	}

	fn handle_event(
		&mut self,
		event: &Event,
		_cx: &mut Context,
	) -> EventResult {
		if self.active_error.is_none() {
			return EventResult::Ignored(None);
		}

		match event {
			Event::Key(kc) => {
				if let Some(LoggingCommand::Acknowledge) = self.keymap.feed(*kc)
				{
					self.close_error();
					return EventResult::Consumed(None);
				}

				// force user to acknowledge error
				EventResult::Consumed(None)
			}
			_ => EventResult::Ignored(None),
		}
	}

	fn should_update(&self) -> bool {
		self.dirty
	}
}
